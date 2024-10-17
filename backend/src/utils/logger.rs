use std::fmt::Debug;
use std::format;
use std::path::Path;

use anyhow::Result;
use chrono::Utc;
use colorful::Colorful;
use log::{LevelFilter, Record};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Root};
use log4rs::config::runtime::Logger as LogLogger;
use log4rs::encode::{Encode, Write};
use log4rs::Handle;

use crate::{tui_color, tui_error, tui_info, tui_success};
use crate::utils::config::Config;

pub struct Logger {
    config: Config,
}

impl Logger {
    pub fn from(config: Config) -> Self {
        Logger { config }
    }

    pub fn init(&self) -> Result<Option<Handle>> {
        let guards = self.build();
        match &guards {
            Ok(Some(_)) => {
                let output = [
                    "activated ",
                    match self.config.console {
                        true => "console",
                        false => "",
                    },
                    match self.config.console && self.config.logfile {
                        true => ",",
                        false => "",
                    },
                    match self.config.logfile {
                        true => "logfile",
                        false => "",
                    },
                    " (level=",
                    self.config.loglevel.as_str(),
                    ")",
                ];
                tui_success!("Logger ready", output.concat());
            }
            Ok(None) => {
                tui_info!("All loggers are disabled");
            }
            Err(err) => {
                tui_error!("Logger failed to be initialized", err.to_string());
            }
        };
        guards
    }

    fn build(&self) -> Result<Option<Handle>> {
        let mut log_builder = log4rs::Config::builder();
        // Root logger: will log nothing.
        let root_logger = Root::builder();
        // Hermes-Studio logger: will log on stderr and console appender according to configuration.
        let mut hermes_studio_logger = LogLogger::builder();
        // Hermes-Five logger: will log on stderr and console appender according to configuration.
        let mut hermes_five_logger = LogLogger::builder();

        // =======================================================================
        // Build and use a stderr log appender: log in the console.
        if self.config.console {
            log_builder = log_builder.appender(
                Appender::builder().build(
                    "stderr",
                    Box::new(
                        ConsoleAppender::builder()
                            .encoder(Box::new(HermesEncoder::default().with_ansi(true)))
                            .target(Target::Stderr)
                            .build(),
                    ),
                ),
            );
            // Use this in our custom loggers.
            hermes_studio_logger = hermes_studio_logger.appender("stderr");
            hermes_five_logger = hermes_five_logger.appender("stderr");
        }

        // =======================================================================
        // Build and use a logfile appender: log in file.
        if self.config.logfile {
            let log_path = Path::new(&self.config.logfile_path);
            let trigger = Box::new(SizeTrigger::new(2 * 1024 * 1024)); // 2MB
            let roller = Box::new(
                FixedWindowRoller::builder().base(1).build(
                    log_path
                        .parent()
                        .unwrap()
                        .join("debug.ARCHIVE-{}.log")
                        .to_str()
                        .unwrap(),
                    5,
                )?,
            );
            let compound_policy = Box::new(CompoundPolicy::new(trigger, roller));
            log_builder = log_builder.appender(
                Appender::builder().build(
                    "logfile",
                    Box::new(
                        RollingFileAppender::builder()
                            .encoder(Box::new(HermesEncoder::default().with_ansi(false)))
                            .build(log_path, compound_policy)?,
                    ),
                ),
            );
            // Use this in our custom loggers.
            hermes_studio_logger = hermes_studio_logger.appender("logfile");
            hermes_five_logger = hermes_five_logger.appender("logfile");
        }

        // =======================================================================
        // Set logging level for 'hermes-studio' and use it.
        log_builder = log_builder.logger(
            hermes_studio_logger
                .additive(true)
                .build("hermes_studio", self.config.loglevel.to_level_filter()),
        );

        // =======================================================================
        // Set logging level for 'hermes-five' and use it.
        log_builder = log_builder.logger(
            hermes_five_logger
                .additive(true)
                .build("hermes_five", self.config.loglevel.to_level_filter()),
        );

        // =======================================================================
        if !self.config.console && !self.config.logfile {
            return Ok(None);
        }

        // Disable logs for all other crates.
        let logger = log_builder.build(root_logger.build(LevelFilter::Off))?;
        let guard = log4rs::init_config(logger)?;
        Ok(Some(guard))
    }
}

/// Custom Encoder for the project, usable into log4rs.
/// This encoder adds support for styling through colorful crate.
/// It can enable/disable the colorization through an optional ansi_support flag.
#[derive(Debug, Default)]
struct HermesEncoder {
    ansi_support: bool,
}

impl HermesEncoder {
    fn with_ansi(mut self, ansi: bool) -> Self {
        self.ansi_support = ansi;
        self
    }
}

impl Encode for HermesEncoder {
    fn encode(&self, w: &mut dyn Write, record: &Record) -> Result<()> {
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        let message = match self.ansi_support {
            true => format!(
                "{} | {} | {} \n",
                format!(
                    "{} — {}:{}",
                    Utc::now().format("%Y-%m-%d %H:%M:%S"),
                    record.file().unwrap(),
                    record.line().unwrap()
                )
                .dim()
                .italic(),
                tui_color!(record.level(), "{:5.5}", record.level()),
                tui_color!(record.level(), "{}", record.args())
            ),
            false => format!(
                "{} — {}:{} | {:5.5} | {}\n",
                Utc::now().format("%Y-%m-%d %H:%M:%S"),
                record.file().unwrap(),
                record.line().unwrap(),
                record.level(),
                record.args()
            ),
        };
        w.write_all(message.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{File, remove_file};
    use std::path::PathBuf;

    use log::Level;

    use crate::utils::config::Config;
    use crate::utils::logger::Logger;

    #[test]
    fn test_logger_file() {
        let log_path = "../../logs/tests/test_logger_file.log";
        let _removed = remove_file(log_path);
        let mut config = Config::default();
        config.console = true;
        config.loglevel = Level::Trace;
        config.logfile = true;
        config.logfile_path = PathBuf::from(log_path);
        let logger = Logger::from(config).init();
        assert!(logger.is_ok());
        assert!(logger.unwrap().is_some());
        assert!(std::fs::read_to_string(log_path)
            .unwrap()
            .contains("Logger ready: activated console,logfile (level=TRACE)"));
    }

    #[test]
    fn test_disabled_logger() {
        let log_path = "../../logs/tests/test_disabled_logger.log";
        let _removed = remove_file(log_path);
        let mut config = Config::default();
        config.console = false;
        config.loglevel = Level::Trace;
        config.logfile = false;
        config.logfile_path = PathBuf::from(log_path);
        let logger = Logger::from(config).init();
        assert!(logger.is_ok());
        assert!(logger.unwrap().is_none());
        assert!(File::open(log_path).is_err());
    }

    #[test]
    fn test_failing_logger() {
        let mut config = Config::default();
        config.logfile = true;
        config.logfile_path = PathBuf::from(".");
        let logger = Logger::from(config).init();
        assert!(logger.is_err());
    }
}
