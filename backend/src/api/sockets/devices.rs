use anyhow::{anyhow, bail};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::extract::{AckSender, Data, SocketRef};

use crate::api::sockets::broadcast_and_ack;
use crate::hardware::boards::Board;
use crate::hardware::devices::led::LedDevice;
use crate::hardware::devices::servo::ServoDevice;
use crate::hardware::devices::{Device, DeviceType};
use crate::hardware::HardwareId;
use crate::storage::entity::{Entity, Id};

#[derive(Debug, Serialize, Deserialize)]
struct DevicePayload {
    name: String,
    config: Box<dyn DeviceType>,
}

pub fn register_device_events(socket: &SocketRef) {
    // Build a device of a given type.
    // This device is not saved.
    // @todo: remove !!
    socket.on(
        "device:build",
        |socket: SocketRef, Data(value): Data<String>, ack: AckSender| {
            debug!("Event received: [device:generate]: {:?}", value);
            // @todo: make that discoverable via a factory with hardware registration (roadmap: plugins)
            let instance = match value.as_str() {
                "LedDevice" => Ok(LedDevice::default_instance()),
                "ServoDevice" => Ok(ServoDevice::default_instance()),
                _ => Err(anyhow!("Unknown structure type")),
            };
            broadcast_and_ack("device:updated", instance, socket, ack);
        },
    );

    socket.on(
        "device:create",
        |socket: SocketRef, Data(value): Data<Value>, ack: AckSender| {
            debug!("Event received: [device:create]: {:?}", value);
            let bid = value[0].as_u64().unwrap() as Id;
            let device = serde_json::from_value::<Device>(value[1].clone()).unwrap();
            let device = Board::get(&bid).and_then(|board| match board {
                None => bail!("Board not found"),
                Some(mut board) => {
                    let device = board.with_device(device);
                    board.save()?;
                    Ok(device)
                }
            });
            broadcast_and_ack("device:updated", device, socket, ack);
        },
    );

    socket.on(
        "device:update",
        |socket: SocketRef, Data(value): Data<Value>, ack: AckSender| {
            debug!("Event received: [device:update]: device:{:?}", value);
            let bid = value[0].as_u64().unwrap() as Id;
            let device = serde_json::from_value::<Device>(value[1].clone()).unwrap();
            let device = Board::get(&bid).and_then(|board| match board {
                None => bail!("Board not found"),
                Some(mut board) => {
                    let device = board.with_device(device);
                    board.save()?;
                    Ok(device)
                }
            });
            broadcast_and_ack("device:updated", device, socket, ack);
        },
    );

    socket.on(
        "device:delete",
        |socket: SocketRef, Data(value): Data<Value>, ack: AckSender| {
            debug!("Event received: [device:delete]: id:{:?}", value);
            let bid = value[0].as_u64().unwrap() as Id;
            let id = value[1].as_u64().unwrap() as HardwareId;

            let device = Board::get(&bid).and_then(|board| match board {
                None => bail!("Board not found"),
                Some(mut board) => match board.remove_device(&id) {
                    None => bail!("Device not found"),
                    Some(device) => {
                        board.save()?;
                        return Ok(device);
                    }
                },
            });

            broadcast_and_ack("device:deleted", device, socket, ack);
        },
    );
}
