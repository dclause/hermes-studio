//! This file provides general routes and handlers for CRUD operations regarding `Board`s specifically.

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use crate::api::AppState;
use crate::hardware::Hardware;

/// Consolidates all available REST API routes for `Board`.
pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/", get(handler_boards_list)) //.post(handler_create_board))
}

/// GET /:version/hardwares.
/// Retrieves all hardwares information.
async fn handler_boards_list(State(state): State<AppState>) -> impl IntoResponse {
    let hardwares = state.database.read().list::<Hardware>().unwrap();
    Json(hardwares)
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
