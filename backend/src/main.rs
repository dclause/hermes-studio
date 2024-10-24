use std::process::exit;

use anyhow::Result;

use crate::app::App;

mod animation;
mod api;
mod app;
mod extra;
mod hardware;
mod server;
mod utils;

/// Entry point for the project.
fn main() -> Result<()> {
    if let Err(err) = App::run() {
        eprintln!("Program failed: {err}");
        exit(1);
    }
    Ok(())
}
