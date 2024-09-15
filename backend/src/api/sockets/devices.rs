use anyhow::{anyhow, bail};
use hermes_five::utils::Easing;
use log::debug;
use socketioxide::extract::{AckSender, Data, SocketRef, State, TryData};

use crate::animation::group::Group;
use crate::api::sockets::{broadcast_and_ack, broadcast_to_all};
use crate::api::sockets::ack::Ack;
use crate::hardware::board::Board;
use crate::hardware::device::Device;
use crate::utils::database::ArcDb;
use crate::utils::entity::{Entity, Id};

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
         ack: AckSender| {
            debug!(
                "Event received: [device:mutate]: device={}, state={:?}",
                id, state
            );
            database.write().set_autosave(false);

            let mutation = Device::get(&database, &id).and_then(|device| match device {
                None => bail!("Device not found"),
                Some(mut device) => {
                    device
                        .inner
                        .set_state(state)
                        .and_then(|state| match device.save(&database) {
                            Ok(_) => Ok(state),
                            Err(err) => bail!(err.to_string()),
                        })
                }
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

            database.write().set_autosave(true);
            ack.send(Ack::from(mutation)).ok();
        },
    );

    socket.on(
        "device:reset",
        |socket: SocketRef, database: State<ArcDb>, Data(id): Data<Id>| {
            debug!("Event received: [device:reset]: {:?}", id);
            database.write().set_autosave(false);

            let mutation = Device::get(&database, &id).and_then(|device| match device {
                None => bail!("Device not found"),
                Some(mut device) => {
                    device
                        .inner
                        .reset()
                        .and_then(|state| match device.save(&database) {
                            Ok(_) => Ok((id, state)),
                            Err(err) => bail!(err.to_string()),
                        })
                }
            });
            database.write().set_autosave(true);
            broadcast_to_all("device:mutated", mutation, &socket);
        },
    );

    socket.on(
        "device:animate",
        |socket: SocketRef,
         State(database): State<ArcDb>,
         Data((id, state, duration, transition)): Data<(Id, u16, u64, Easing)>,
         ack: AckSender| {
            debug!(
                "Event received: [device:animate]: device={}, state={:?}, duration={}, transition={:?}",
                id, state, duration, transition
            );

            let mutation = Device::get(&database, &id).and_then(|device| match device {
                None => bail!("Device not found"),
                Some(mut device) => device.inner.animate(state, duration, transition),
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
         TryData(new_device): TryData<Device>,
         ack: AckSender| {
            debug!("Event received: [device:create]: {:#?}", new_device);

            let device = match new_device {
                Err(error) => Err(anyhow!("Invalid device: {}", error)),
                Ok(mut new_device) => {
                    Board::get(&database, &new_device.bid).and_then(|board| match board {
                        None => bail!("Board [{}] not found", new_device.bid),
                        Some(board) => {
                            if board.connected {
                                new_device.inner.set_board(&board)?;
                            }
                            database.write().insert(new_device)
                        }
                    })
                }
            };

            broadcast_to_all("group:list", database.read().list::<Group>(), &socket);
            broadcast_and_ack("device:updated", device, &socket, ack);
        },
    );

    socket.on(
        "device:update",
        |socket: SocketRef,
         State(database): State<ArcDb>,
         TryData(device): TryData<Device>,
         ack: AckSender| {
            debug!("Event received: [device:update]: {:#?}", device);

            let device = match device {
                Err(error) => Err(anyhow!("Invalid device: {}", error)),
                Ok(mut device) => {
                    Board::get(&database, &device.bid).and_then(|board| match board {
                        None => bail!("Board [{}] not found", device.bid),
                        Some(board) => {
                            if board.connected {
                                device.inner.set_board(&board)?;
                            }
                            database.write().update(device)
                        }
                    })
                }
            };

            broadcast_and_ack("device:updated", device, &socket, ack);
        },
    );

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
