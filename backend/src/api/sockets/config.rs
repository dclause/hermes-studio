use log::debug;
use serde_json::Value;
use socketioxide::extract::{AckSender, Data, SocketRef, State};

use crate::api::sockets::ack::Ack;
use crate::api::sockets::broadcast_and_ack;
use crate::utils::database::ArcDb;
use crate::utils::interface::Interface;

pub fn register_config_events(socket: &SocketRef) {
    socket.on(
        "config:get",
        |ack: AckSender, State(database): State<ArcDb>| {
            debug!("Event received: [config:get]");
            let config = Interface::get_config_from_db(database);
            ack.send(Ack::from(config)).ok();
        },
    );

    socket.on(
        "config:set",
        |socket: SocketRef,
         State(database): State<ArcDb>,
         Data(config): Data<Value>,
         ack: AckSender| {
            debug!("Event received: [config:set]");
            let config = Interface::set_config_to_db(database, config);
            broadcast_and_ack("config:updated", config, &socket, ack);
        },
    );
}
