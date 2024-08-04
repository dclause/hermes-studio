//! This module contains all the code related to the SocketIO events handling.

use log::info;
use serde::Serialize;
use socketioxide::extract::{AckSender, SocketRef};
use socketioxide::layer::SocketIoLayer;
use socketioxide::SocketIo;

use crate::api::sockets::ack::Ack;
use crate::api::sockets::config::register_config_events;

// use crate::api::sockets::animations::register_animation_events;
// use crate::api::sockets::boards::register_board_events;
// use crate::api::sockets::devices::register_device_events;

pub mod ack;
mod config;
// mod animations;
// mod boards;
// mod devices;

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

pub(crate) fn build_socket_routes(
    register_events: Vec<fn(socket: &SocketRef)>,
) -> (SocketIoLayer, SocketIo) {
    let (layer, io) = SocketIo::builder().build_layer();
    io.ns("/ws", move |socket: SocketRef| {
        info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

        register_config_events(&socket);
        // register_board_events(&socket);
        // register_device_events(&socket);
        // register_animation_events(&socket);

        for register_event in &register_events {
            register_event(&socket);
        }
    });
    (layer, io)
}
