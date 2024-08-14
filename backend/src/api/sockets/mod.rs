//! This module contains all the code related to the SocketIO events handling.

use serde::Serialize;
use socketioxide::extract::{AckSender, SocketRef};

use crate::api::sockets::ack::Ack;
use crate::api::sockets::boards::register_board_events;
use crate::api::sockets::config::register_config_events;
use crate::api::sockets::devices::register_device_events;
use crate::api::sockets::groups::register_group_events;

// use crate::api::sockets::animations::register_animation_events;
// use crate::api::sockets::boards::register_board_events;
// use crate::api::sockets::devices::register_device_events;

pub mod ack;
mod boards;
mod config;
mod devices;
mod groups;

/// Helper function: broadcast the value and send ack.
pub fn broadcast_and_ack<T: Serialize>(
    event: &'static str,
    data: anyhow::Result<T>,
    socket: SocketRef,
    ack: AckSender,
) {
    if data.is_ok() {
        socket.broadcast().emit(event, data.as_ref().unwrap()).ok();
    }
    ack.send(Ack::from(data)).ok();
}

pub fn register_socket_events(
    socket: SocketRef,
    custom_register_callbacks: Vec<fn(socket: &SocketRef)>,
) {
    register_config_events(&socket);
    register_board_events(&socket);
    register_device_events(&socket);
    register_group_events(&socket);
    // register_animation_events(&socket);

    for custom_register in &custom_register_callbacks {
        custom_register(&socket);
    }
}
