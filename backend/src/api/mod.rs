//! This module contains all the code related to the server APIs: both REST and SocketIO.
//!
//! This server lets you interact with your Hermes-Five robot from the internet, possibly from a UI.
//! This API is currently implemented using `axum` crate.
use socketioxide::SocketIo;

use crate::utils::database::ArcDb;

mod payloads;
pub mod rest;
pub mod sockets;

#[derive(Clone)]
pub struct AppState {
    pub database: ArcDb,
    pub socket: SocketIo,
}
