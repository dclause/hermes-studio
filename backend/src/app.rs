use anyhow::Result;
use clap::Parser;
use colorful::Colorful;

use crate::{tui_info, tui_opening};
use crate::utils::cli::CliArgs;
use crate::utils::config::Config;

pub struct App;

impl App {
    /// Starts the application.
    #[hermes_five::runtime]
    pub async fn run() -> Result<()> {
        // Parse cli args: handle `help`, `version`, etc...
        let args = CliArgs::parse();

        tui_opening!();

        // Build configuration and save it globally.
        Config::from(args)?.save();

        tui_info!("Application is now stopped");
        Ok(())
    }
}
