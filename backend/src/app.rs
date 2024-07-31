use anyhow::Result;
use colorful::Colorful;

use crate::{tui_info, tui_opening};

pub struct App;

impl App {
    // pub fn get_config(config_path: &'static str) -> Config {
    //     match Config::from(config_path) {
    //         Ok(config) => {
    //             tui_success!("Configuration initialized from file", config_path);
    //             config
    //         }
    //         Err(err) => {
    //             tui_info!(
    //                 "Default configuration used",
    //                 err.to_string().split('\n').next().unwrap()
    //             );
    //             Config::default()
    //         }
    //     }
    // }

    /// Starts the application.
    #[hermes_five::runtime]
    pub async fn run() -> Result<()> {
        tui_opening!();

        tui_info!("Application is now stopped");
        Ok(())
    }
}
