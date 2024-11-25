//! This file provides general routes and handlers for CRUD operations regarding `Board`s specifically.

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use std::collections::HashMap;

use crate::api::payloads::BoardPayload;
use crate::api::AppState;
use crate::hardware::Board;
use crate::utils::entity::Id;

/// Consolidates all available REST API routes for `Board`.
pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/", get(handler_boards_list)) //.post(handler_create_board))
}

/// GET /:version/boards.
/// Retrieves all boards information.
async fn handler_boards_list(State(state): State<AppState>) -> impl IntoResponse {
    let boards = state.database.read().list::<Board>().unwrap();
    let payloads = boards
        .into_iter()
        .map(|(id, board)| (id, BoardPayload::from(board)))
        .collect::<HashMap<Id, BoardPayload>>();
    Json(payloads)
}

// /// POST /:version/hardwares.
// /// Create a new board.
// async fn handler_create_board(
//     State(state): State<AppState>,
//     Json(payload): Json<CreateBoard>,
// ) -> impl IntoResponse {
//     let board: Board = payload.into();
//     let board = state.database.write().insert(board).unwrap();
//     Json(board)
// }
