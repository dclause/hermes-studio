use anyhow::{anyhow, bail};
use log::debug;
use socketioxide::extract::{AckSender, Data, SocketRef, State, TryData};

use crate::api::payloads::board::CreateBoard;
use crate::api::sockets::ack::Ack;
use crate::api::sockets::broadcast_and_ack;
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
            let board = Board::get(&database, &id)
                .and_then(|board| match board {
                    None => bail!("Board not found"),
                    Some(board) => board.open(),
                })
                .unwrap();

            // Initialize properly the inner device value (because now that board is open(), the
            // handshake as given us the hardware board configuration, which let's us properly initialize
            // our devices.
            let devices = database.read().list::<Device>().unwrap();
            for (id, mut device) in devices {
                if device.bid == board.id {
                    device.inner.init(&board).unwrap();
                    device.save(&database).unwrap();
                }
            }

            let board = board.save(&database);
            broadcast_and_ack("board:updated", board, socket, ack);
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
            broadcast_and_ack("board:updated", board, socket, ack);
        },
    );

    socket.on(
        "board:add",
        |socket: SocketRef,
         TryData(new_board): TryData<CreateBoard>,
         database: State<ArcDb>,
         ack: AckSender| {
            debug!("Event received: [board:add]: board:{:?}", new_board);

            let board = match new_board {
                Ok(new_board) => {
                    let board: Board = new_board.into();
                    database.write().insert(board)
                }
                Err(error) => Err(anyhow!("Invalid board: {}", error)),
            };

            broadcast_and_ack("board:updated", board, socket, ack);
        },
    );
    //
    // socket.on(
    //     "board:update",
    //     |socket: SocketRef, TryData(board): TryData<Board>, ack: AckSender| {
    //         debug!("Event received: [board:update]: board:{:?}", board);
    //
    //         let board = match board {
    //             Ok(board) => board.save(),
    //             Err(error) => Err(anyhow!("Invalid board: {}", error)),
    //         };
    //
    //         broadcast_and_ack("board:updated", board, socket, ack);
    //     },
    // );
    //
    // socket.on(
    //     "board:delete",
    //     |socket: SocketRef, TryData(id): TryData<Id>, ack: AckSender| {
    //         debug!("Event received: [board:delete]: id:{:?}", id);
    //         let board = Board::delete_by_id(id.unwrap()).and_then(|board| match board {
    //             None => bail!("Board not found"),
    //             Some(board) => Ok(board),
    //         });
    //         broadcast_and_ack("board:deleted", board, socket, ack);
    //     },
    // );
}
