//! This file contains code relative to configuration within the application.
use std::net::IpAddr;
use std::path::PathBuf;

use anyhow::Result;
use figment::Figment;
use figment::providers::{Env, Format, Serialized, Toml};
use log::Level;
use serde::{Deserialize, Serialize};

use crate::utils::cli::CliArgs;

/// Consolidated Config structure to be exposed globally throughout the application.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Defines the logger level configuration for the application.
    pub loglevel: Level,
    /// Activates logs on console if set to TRUE.
    pub console: bool,
    /// Activates logs to file if set to TRUE.
    pub logfile: bool,
    /// Customizes the path to log file.
    pub logpath: PathBuf,
    /// Host to expose the application.
    pub host: IpAddr,
    /// Port to expose the application.
    pub port: u16,
    /// The database path.
    pub database_path: PathBuf,
    /// The website path.
    pub website_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let mut current_path = std::env::current_exe().unwrap();
        current_path.pop();
        Config {
            loglevel: Level::Error,
            console: false,
            logfile: false,
            logpath: current_path.join("logs/debug.log"),
            host: IpAddr::from([0, 0, 0, 0]),
            port: 4000,
            database_path: current_path.join("database"),
            website_path: current_path.join("website"),
        }
    }
}

impl Config {
    /// Initializes the global configuration.
    ///
    /// Configurations are set by order of priorities:
    ///  - cli args
    ///  - env variables
    ///  - toml file (depending on profile, by default "configs/default")
    ///  - default configuration
    pub fn from(args: CliArgs) -> Result<Config> {
        // TOML file path.
        let config_path = PathBuf::from("./configs").join("config.toml");

        let config: Config = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file(config_path))
            .merge(Env::prefixed("HERMES_"))
            .merge(Serialized::defaults(args))
            .extract()?;

        log::debug!("Configuration built:\n{:?}", config);
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use clap::Parser;

    use super::*;

    #[test]
    fn test_build_config() {
        // Default configuration
        let config = Config::from(CliArgs::parse_from(&["test"]));
        assert!(config.is_ok());
        assert_eq!(config.unwrap().port, 4000, "Default configuration");

        // Overridden by ENV
        env::set_var("HERMES_PORT", "6000");
        let config = Config::from(CliArgs::parse_from(&["test"]));
        assert!(config.is_ok());
        assert_eq!(config.unwrap().port, 6000, "ENV configuration");

        // Overridden by cli
        let config = Config::from(CliArgs::parse_from(&["test", "--port", "7000"]));
        assert!(config.is_ok());
        assert_eq!(config.unwrap().port, 7000, "CLI configuration");
    }
}
