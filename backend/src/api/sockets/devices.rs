use anyhow::bail;
use log::debug;
use socketioxide::extract::{AckSender, Data, SocketRef, State, TryData};

use crate::animation::groups::Group;
use crate::api::sockets::{broadcast_and_ack, broadcast_to_all};
use crate::api::sockets::ack::Ack;
use crate::hardware::board::Board;
use crate::hardware::device::Device;
use crate::utils::database::ArcDb;
use crate::utils::entity::{Entity, Id};

// #[derive(Debug, Serialize, Deserialize)]
// struct DevicePayload {
//     name: String,
//     config: Box<dyn DeviceType>,
// }

pub fn register_device_events(socket: &SocketRef) {
    socket.on(
        "device:list",
        |State(database): State<ArcDb>, ack: AckSender| {
            debug!("Event received: [device:list]");
            let devices = database.read().list::<Device>();
            ack.send(Ack::from(devices)).ok();
        },
    );

    socket.on(
        "device:mutate",
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
                    .emit("device:mutated", (id, mutation.as_ref().unwrap()))
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
                broadcast_to_all("board:updated", board, &socket);
            }
            ack.send(Ack::from(mutation)).ok();
        },
    );

    socket.on(
        "device:create",
        |socket: SocketRef,
         State(database): State<ArcDb>,
         TryData(data): TryData<Device>,
         ack: AckSender| {
            debug!("Event received: [device:create]: {:?}", data);

            if data.is_err() {
                ack.send(Ack::from(data.map_err(anyhow::Error::msg))).ok();
                return;
            }

            let mut new_device = data.unwrap();
            let device = Board::get(&database, &new_device.bid).and_then(|board| match board {
                None => new_device.save(&database),
                Some(board) => {
                    if board.connected {
                        new_device.inner.reset(&board)?;
                    }
                    new_device.save(&database)
                }
            });

            broadcast_and_ack("device:updated", device, &socket, ack);
        },
    );
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
    socket.on(
        "device:delete",
        |socket: SocketRef, database: State<ArcDb>, Data(id): Data<Id>, ack: AckSender| {
            debug!("Event received: [device:delete]: id:{:?}", id);
            let device = database
                .write()
                .delete::<Device>(id)
                .and_then(|device| match device {
                    None => bail!("Device not found"),
                    Some(device) => Ok(device),
                });

            // Groups have been updated:
            let groups = database.read().list::<Group>();
            broadcast_to_all("groups:updated", groups, &socket);

            broadcast_and_ack("device:deleted", device, &socket, ack);
        },
    );
}
