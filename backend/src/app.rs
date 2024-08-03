use anyhow::Result;
use clap::Parser;
use colorful::Colorful;

use crate::{tui_error, tui_info, tui_opening, tui_success};
use crate::api::server::Server;
use crate::utils::cli::CliArgs;
use crate::utils::config::Config;
use crate::utils::logger::Logger;

pub struct App;

impl App {
    /// Starts the application.
    #[hermes_five::runtime]
    pub async fn run() -> Result<()> {
        // Parse cli args: handle `help`, `version`, etc...
        let args = CliArgs::parse();

        tui_opening!();

        // Build configuration and save it globally.
        let config = Config::from(args)?;

        // Build and run the logger
        Logger::from(config.clone()).init()?;

        // Build the server.
        let server = Server::from(config);

        if let Err(err) = server.start().await {
            tui_error!("An error occurred:", err.to_string());
        }

        tui_info!("Application is now stopped");
        Ok(())
    }
}
