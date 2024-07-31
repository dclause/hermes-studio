use std::process::exit;

use anyhow::Result;

use crate::app::App;

mod app;
mod utils;

/// Entry point for the project.
fn main() -> Result<()> {
    if let Err(err) = App::run() {
        eprintln!("Program failed: {err}");
        exit(1);
    }
    Ok(())
}

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

/*pub fn init_storage(folder: &'static str) {
    match Storage::init_persistent(folder, false, true) {
        Ok(file_storage) => {
            tui_success!("Storage ready - saving to", folder);
            file_storage
        }
        Err(err) => {
            tui_warn!(
                "No persistent storage created - all information will be lost",
                err.to_string().split('\n').next().unwrap()
            );
            Storage::init_volatile().unwrap();
        }
    }
}*/

// pub run() {
//     // hierarchical config. cli args override envars which override toml config values
//     let conf: Config = Figment::new()
//     .merge(Toml::file("Config.toml"))
//     .merge(Env::prefixed("APP_"))
//     .merge(Serialized::defaults(Cli::parse()))
//     .extract()?;
// }
