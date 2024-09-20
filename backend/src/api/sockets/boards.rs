use anyhow::{anyhow, bail};
use log::debug;
use socketioxide::extract::{AckSender, Data, SocketRef, State, TryData};

use crate::animation::group::Group;
use crate::api::sockets::{broadcast_and_ack, broadcast_to_all};
use crate::api::sockets::ack::Ack;
use crate::hardware::board::Board;
use crate::hardware::device::Device;
use crate::utils::database::ArcDb;
use crate::utils::entity::{Entity, Id};

pub fn register_board_events(socket: &SocketRef) {
    socket.on(
        "board:list",
        |State(database): State<ArcDb>, ack: AckSender| {
            debug!("Event received: [board:list]");
            let boards = database.read().list::<Board>();
            ack.send(Ack::from(boards)).ok();
        },
    );

    socket.on(
        "board:open",
        |socket: SocketRef, State(database): State<ArcDb>, Data(id): Data<Id>, ack: AckSender| {
            debug!("Event received: [board:open]: board:{}", id);
            let board = Board::get(&database, &id).and_then(|board| match board {
                None => bail!("Board not found"),
                Some(board) => board.open(&database)?.save(&database),
            });
            broadcast_and_ack("board:updated", board, &socket, ack);
        },
    );

    socket.on(
        "board:close",
        |socket: SocketRef, State(database): State<ArcDb>, Data(id): Data<Id>, ack: AckSender| {
            debug!("Event received: [board:close]: board:{}", id);
            let board = Board::get(&database, &id).and_then(|board| match board {
                None => bail!("Board not found"),
                Some(board) => board.close()?.save(&database),
            });
            broadcast_and_ack("board:updated", board, &socket, ack);
        },
    );

    socket.on(
        "board:reset",
        |socket: SocketRef, State(database): State<ArcDb>, Data(id): Data<Id>| {
            debug!("Event received: [board:reset]: board:{}", id);

            database.write().set_autosave(false);
            let devices = database.read().list::<Device>().and_then(|mut devices| {
                for (_, device) in &mut devices {
                    if device.bid == id {
                        device.inner.reset().and_then(|mutation| {
                            broadcast_to_all("device:mutated", Ok((device.id, mutation)), &socket);
                            Ok(())
                        })?;
                    }
                }

                Ok(devices)
            });
            database.write().set_autosave(true);
            // broadcast_to_all("device:list", devices, &socket);
        },
    );

    socket.on(
        "board:create",
        |socket: SocketRef,
         TryData(new_board): TryData<Board>,
         database: State<ArcDb>,
         ack: AckSender| {
            debug!("Event received: [board:create]: board:{:#?}", new_board);

            let board = match new_board {
                Err(error) => Err(anyhow!("Invalid board: {}", error)),
                Ok(new_board) => database.write().insert(new_board),
            };
            broadcast_and_ack("board:updated", board, &socket, ack);
        },
    );

    socket.on(
        "board:update",
        |socket: SocketRef,
         TryData(board): TryData<Board>,
         database: State<ArcDb>,
         ack: AckSender| {
            debug!("Event received: [board:update]: board:{:#?}", board);

            let board = match board {
                Err(error) => Err(anyhow!("Invalid board: {}", error)),
                Ok(board) => {
                    Board::get(&database, &board.id).and_then(|existing_board| match existing_board
                    {
                        None => bail!("Board [{}] not found", board.id),
                        Some(_) => database.write().update(board),
                    })
                }
            };
            broadcast_and_ack("board:updated", board, &socket, ack);
        },
    );

    socket.on(
        "board:delete",
        |socket: SocketRef, database: State<ArcDb>, Data(id): Data<Id>, ack: AckSender| {
            debug!("Event received: [board:delete]: id:{:?}", id);
            let board = database
                .write()
                .delete::<Board>(id)
                .and_then(|board| match board {
                    None => bail!("Board not found"),
                    Some(board) => Ok(board),
                });

            let devices = database.read().list::<Device>();
            broadcast_to_all("device:list", devices, &socket);
            let groups = database.read().list::<Group>();
            broadcast_to_all("group:list", groups, &socket);
            broadcast_and_ack("board:deleted", board, &socket, ack);
        },
    );
}
