use anyhow::{anyhow, bail};
use axum::{debug_handler, Json};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::extract::{AckSender, Data, SocketRef, State, TryData};

use crate::api::{AppState, ArcDb};
use crate::api::payloads::board::CreateBoard;
use crate::api::sockets::ack::Ack;
use crate::api::sockets::broadcast_and_ack;
use crate::hardware::board::Board;
use crate::utils::database::Database;

pub fn register_board_events(socket: &SocketRef) {
    socket.on(
        "board:list",
        |socket: SocketRef, database: State<ArcDb>, ack: AckSender| {
            debug!("Event received: [board:list]");
            let boards = database.read().list::<Board>();
            ack.send(Ack::from(boards)).ok();
        },
    );
    //
    // socket.on(
    //     "board:mutate",
    //     |socket: SocketRef, Data(value): Data<Value>, ack: AckSender| async move {
    //         let board = value[0].as_u64().unwrap() as Id;
    //         let device = value[1].as_u64().unwrap() as HardwareId;
    //         let state = DeviceState::from(value[2].clone());
    //         debug!(
    //             "Event received: [board:mutate]: board:{}, device={}, state={:?}",
    //             board, device, state
    //         );
    //
    //         let mutation = Board::get(&board).and_then(|board| match board {
    //             None => bail!("Board not found"),
    //             Some(mut board) => board.mutate(&device, state),
    //         });
    //
    //         if mutation.is_ok() {
    //             socket
    //                 .broadcast()
    //                 .emit("board:mutated", (board, device, mutation.as_ref().unwrap()))
    //                 .ok();
    //         }
    //         ack.send(Ack::from(mutation)).ok();
    //     },
    // );
    //
    // socket.on(
    //     "board:open",
    //     |socket: SocketRef, Data(id): Data<Id>, ack: AckSender| {
    //         debug!("Event received: [board:open]: board:{}", id);
    //         let board = Board::get(&id).and_then(|board| match board {
    //             None => bail!("Board not found"),
    //             Some(mut board) => {
    //                 board.open()?;
    //                 board.save()
    //             }
    //         });
    //         broadcast_and_ack("board:updated", board, socket, ack);
    //     },
    // );
    //
    // socket.on(
    //     "board:close",
    //     |socket: SocketRef, Data(id): Data<Id>, ack: AckSender| {
    //         debug!("Event received: [board:close]: board:{}", id);
    //         let board = Board::get(&id).and_then(|board| match board {
    //             None => bail!("Board not found"),
    //             Some(mut board) => {
    //                 board.close()?;
    //                 board.save()
    //             }
    //         });
    //         broadcast_and_ack("board:updated", board, socket, ack);
    //     },
    // );
    //
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
