//! This module contains all the code related to the server APIs: both REST and SocketIO.
//!
//! This server lets you interact with your Hermes-Five robot from the internet, possibly from a UI.
//! This API is currently implemented using `axum` crate.
use socketioxide::SocketIo;

use crate::utils::database::Database;

pub mod rest;
pub mod server;
pub mod sockets;

#[derive(Clone)]
pub struct AppState {
    database: Database,
    socket: SocketIo,
}
