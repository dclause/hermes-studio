use log::debug;
use serde_json::Value;
use socketioxide::extract::{AckSender, Data, SocketRef};

use crate::api::sockets::ack::Ack;
use crate::api::sockets::broadcast_and_ack;
use crate::utils::converter::json_to_toml;
use crate::utils::interface::Interface;

pub fn register_config_events(socket: &SocketRef) {
    socket.on("config:get", |ack: AckSender| {
        debug!("Event received: [config:get]");
        let config = Interface::get_config();
        ack.send(Ack::from(config)).ok();
    });

    socket.on(
        "config:set",
        |socket: SocketRef, Data(value): Data<Value>, ack: AckSender| {
            debug!("Event received: [config:set]");
            let config_as_toml: toml::Value = json_to_toml(value).unwrap();
            let config = Interface::set_config(config_as_toml);
            broadcast_and_ack("config:updated", config, &socket, ack);
        },
    );
}
