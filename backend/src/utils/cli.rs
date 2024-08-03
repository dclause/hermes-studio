use std::net::IpAddr;

use clap::Parser;
use log::Level;
use serde::Serialize;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Parser, Serialize)]
#[command(
    name = "Hermes-Studio",
    author,
    version,
    about,
    long_about = "Hermes-Studio CLI launcher"
)]
pub struct CliArgs {
    /// Set log level [default=ERROR].
    #[arg(
        short,
        long,
        global(true),
        value_name = "TRACE|DEBUG|INFO|WARN|ERROR",
        default_value_if("debug", "true", Some("DEBUG"))
    )]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub loglevel: Option<Level>,

    /// Activates debug mode (convenience for `--logLevel="DEBUG"`).
    #[arg(short, long, global(true), value_name = "bool", num_args = 0..=1, default_missing_value = "true")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    debug: Option<bool>,

    /// Outputs logs on console [default=false].
    #[arg(short, long, global(true), value_name = "bool", num_args = 0..=1, default_missing_value = "true")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub console: Option<bool>,

    /// Outputs logs in `logs/debug.log` file [default=false].
    #[arg(short = 'f', long, global(true), value_name = "bool", num_args = 0..=1, default_missing_value = "true")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logfile: Option<bool>,

    /// Server host IP address [default=0.0.0.0].
    #[arg(long, global(true))]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    host: Option<IpAddr>,

    /// Server port [default=5000]..
    #[arg(long, global(true))]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    port: Option<u16>,

    /// Auto open embedded UI in default browser [default=false]..
    #[arg(long, action, global(true), value_name = "bool", num_args = 0..=1, default_missing_value = "true")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    open: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_no_args() {
        let expected = CliArgs {
            loglevel: None,
            debug: None,
            console: None,
            logfile: None,
            host: None,
            port: None,
            open: None,
        };

        let no_args = CliArgs::parse_from(&["test"]);
        assert_eq!(no_args, expected);
    }

    #[test]
    fn test_cli_debug_mode() {
        // Explicit debug to true.
        let args = CliArgs::parse_from(&["test", "--debug", "true"]);
        assert!(args.debug.is_some());
        assert!(args.debug.unwrap());
        assert!(args.loglevel.is_some());
        assert_eq!(args.loglevel.unwrap(), Level::Debug);

        // Explicit debug to false.
        let args = CliArgs::parse_from(&["test", "--debug", "false"]);
        assert!(args.debug.is_some());
        assert!(!args.debug.unwrap());
        assert!(!args.loglevel.is_some());
        assert!(args.loglevel.is_none());

        // Implicit debug value.
        let args = CliArgs::parse_from(&["test", "-d"]);
        assert!(args.debug.is_some());
        assert!(args.debug.unwrap());
        assert!(args.loglevel.is_some());
        assert_eq!(args.loglevel.unwrap(), Level::Debug);
    }

    #[test]
    fn test_cli_log_level() {
        // Log level TRACE
        let args = CliArgs::parse_from(&["test", "--loglevel", "TRACE"]);
        assert!(args.loglevel.is_some());
        assert_eq!(args.loglevel.unwrap(), Level::Trace);

        // Log level DEBUG
        let args = CliArgs::parse_from(&["test", "--loglevel", "DEBUG"]);
        assert!(args.loglevel.is_some());
        assert_eq!(args.loglevel.unwrap(), Level::Debug);

        // Log level INFO
        let args = CliArgs::parse_from(&["test", "-l", "INFO"]);
        assert!(args.loglevel.is_some());
        assert_eq!(args.loglevel.unwrap(), Level::Info);

        // Log level WARN
        let args = CliArgs::parse_from(&["test", "--loglevel", "WARN"]);
        assert!(args.loglevel.is_some());
        assert_eq!(args.loglevel.unwrap(), Level::Warn);

        // Log level ERROR
        let args = CliArgs::parse_from(&["test", "-l", "ERROR"]);
        assert!(args.loglevel.is_some());
        assert_eq!(args.loglevel.unwrap(), Level::Error);
    }

    #[test]
    fn test_cli_log_output_console() {
        // Explicit console to true.
        let args = CliArgs::parse_from(&["test", "--console", "true"]);
        assert!(args.console.is_some());
        assert!(args.console.unwrap());

        // Explicit console to false.
        let args = CliArgs::parse_from(&["test", "--console", "false"]);
        assert!(args.console.is_some());
        assert!(!args.console.unwrap());

        // Implicit console value.
        let args = CliArgs::parse_from(&["test", "-c"]);
        assert!(args.console.is_some());
        assert!(args.console.unwrap());
    }

    #[test]
    fn test_cli_log_output_logfile() {
        // Explicit logfile to true.
        let args = CliArgs::parse_from(&["test", "--logfile", "true"]);
        assert!(args.logfile.is_some());
        assert!(args.logfile.unwrap());

        // Explicit logfile to false.
        let args = CliArgs::parse_from(&["test", "--logfile", "false"]);
        assert!(args.logfile.is_some());
        assert!(!args.logfile.unwrap());

        // Implicit logfile value.
        let args = CliArgs::parse_from(&["test", "-f"]);
        assert!(args.logfile.is_some());
        assert!(args.logfile.unwrap());
    }

    #[test]
    fn test_cli_port() {
        // Explicit open to true.
        let args = CliArgs::parse_from(&["test", "--port", "7000"]);
        assert!(args.port.is_some());
        assert_eq!(args.port.unwrap(), 7000);
    }

    #[test]
    fn test_cli_open_browser() {
        // Explicit open to true.
        let args = CliArgs::parse_from(&["test", "--open", "true"]);
        assert!(args.open.is_some());
        assert!(args.open.unwrap());

        // Explicit open to false.
        let args = CliArgs::parse_from(&["test", "--open", "false"]);
        assert!(args.open.is_some());
        assert!(!args.open.unwrap());

        // Implicit open value.
        let args = CliArgs::parse_from(&["test", "--open"]);
        assert!(args.open.is_some());
        assert!(args.open.unwrap());
    }
}
