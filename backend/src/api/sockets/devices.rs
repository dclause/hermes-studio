use anyhow::bail;
use log::debug;
use serde_json::Value;
use socketioxide::extract::{AckSender, Data, SocketRef, State, TryData};

use crate::api::sockets::ack::Ack;
use crate::hardware::board::Board;
use crate::hardware::device::Device;
use crate::utils::database::ArcDb;
use crate::utils::entity::{Entity, Id};
use crate::utils::entity::private_entity::EntityToAny;

// #[derive(Debug, Serialize, Deserialize)]
// struct DevicePayload {
//     name: String,
//     config: Box<dyn DeviceType>,
// }

pub fn register_device_events(socket: &SocketRef) {
    socket.on(
        "actuator:list",
        |State(database): State<ArcDb>, ack: AckSender| {
            debug!("Event received: [actuator:list]");
            let devices = database.read().list::<Device>();
            ack.send(Ack::from(devices)).ok();
        },
    );

    socket.on(
        "actuator:mutate",
        |socket: SocketRef,
         State(database): State<ArcDb>,
         Data((id, state)): Data<(Id, u16)>,
         ack: AckSender| async move {
            debug!(
                "Event received: [device:mutate]: device={}, state={:?}",
                id, state
            );

            let mutation = Device::get(&database, &id).and_then(|device| match device {
                None => bail!("Device not found"),
                Some(mut device) => device.inner.set_state(state),
            });

            if mutation.is_ok() {
                socket
                    .broadcast()
                    .emit("actuator:mutated", (id, mutation.as_ref().unwrap()))
                    .ok();
            } else {
                let board = Board::get(&database, &id).and_then(|board| match board {
                    None => bail!("Board not found"),
                    Some(mut board) => {
                        board.connected = false;
                        Ok(board)
                    }
                });
                // Update to all, including socket itself myself.
                socket
                    .broadcast()
                    .emit("board:updated", board.as_ref().unwrap())
                    .ok();
                socket.emit("board:updated", board.as_ref().unwrap()).ok();
            }
            ack.send(Ack::from(mutation)).ok();
        },
    );

    // // Build a device of a given type.
    // // This device is not saved.
    // // @todo: remove !!
    // socket.on(
    //     "device:build",
    //     |socket: SocketRef, Data(value): Data<String>, ack: AckSender| {
    //         debug!("Event received: [device:generate]: {:?}", value);
    //         // @todo: make that discoverable via a factory with hardware registration (roadmap: plugins)
    //         let instance = match value.as_str() {
    //             "LedDevice" => Ok(LedDevice::default_instance()),
    //             "ServoDevice" => Ok(ServoDevice::default_instance()),
    //             _ => Err(anyhow!("Unknown structure type")),
    //         };
    //         broadcast_and_ack("device:updated", instance, socket, ack);
    //     },
    // );
    //
    // socket.on(
    //     "device:create",
    //     |socket: SocketRef, Data(value): Data<Value>, ack: AckSender| {
    //         debug!("Event received: [device:create]: {:?}", value);
    //         let bid = value[0].as_u64().unwrap() as Id;
    //         let device = serde_json::from_value::<Device>(value[1].clone()).unwrap();
    //         let device = Board::get(&bid).and_then(|board| match board {
    //             None => bail!("Board not found"),
    //             Some(mut board) => {
    //                 let device = board.with_device(device);
    //                 board.save()?;
    //                 Ok(device)
    //             }
    //         });
    //         broadcast_and_ack("device:updated", device, socket, ack);
    //     },
    // );
    //
    // socket.on(
    //     "device:update",
    //     |socket: SocketRef, Data(value): Data<Value>, ack: AckSender| {
    //         debug!("Event received: [device:update]: device:{:?}", value);
    //         let bid = value[0].as_u64().unwrap() as Id;
    //         let device = serde_json::from_value::<Device>(value[1].clone()).unwrap();
    //         let device = Board::get(&bid).and_then(|board| match board {
    //             None => bail!("Board not found"),
    //             Some(mut board) => {
    //                 let device = board.with_device(device);
    //                 board.save()?;
    //                 Ok(device)
    //             }
    //         });
    //         broadcast_and_ack("device:updated", device, socket, ack);
    //     },
    // );
    //
    // socket.on(
    //     "device:delete",
    //     |socket: SocketRef, Data(value): Data<Value>, ack: AckSender| {
    //         debug!("Event received: [device:delete]: id:{:?}", value);
    //         let bid = value[0].as_u64().unwrap() as Id;
    //         let id = value[1].as_u64().unwrap() as HardwareId;
    //
    //         let device = Board::get(&bid).and_then(|board| match board {
    //             None => bail!("Board not found"),
    //             Some(mut board) => match board.remove_device(&id) {
    //                 None => bail!("Device not found"),
    //                 Some(device) => {
    //                     board.save()?;
    //                     return Ok(device);
    //                 }
    //             },
    //         });
    //
    //         broadcast_and_ack("device:deleted", device, socket, ack);
    //     },
    // );
}
