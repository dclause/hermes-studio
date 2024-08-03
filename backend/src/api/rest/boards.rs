//! This file provides general routes and handlers for CRUD operations regarding `Board`s specifically.

use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use socketioxide::SocketIo;

use crate::hardware::boards::Board;
use crate::storage::storage::Storage;

/// Consolidates all available REST API routes for `Board`.
pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/", get(handler_boards_list))
}

/// GET /:version/boards.
/// Retrieves all boards information.
async fn handler_boards_list() -> impl IntoResponse {
    let boards = Storage::list::<Board>().unwrap();
    Json(boards)
}
