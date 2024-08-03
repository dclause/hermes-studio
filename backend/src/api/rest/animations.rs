//! This file provides general routes and handlers for CRUD operations regarding `Animation`s specifically.

use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use socketioxide::SocketIo;

use crate::animations::animation::Animation;
use crate::storage::storage::Storage;

/// Consolidates all available REST API routes for `Animation`.
pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/", get(handler_animations_list))
}

/// GET /:version/animations.
/// Retrieves all animations information.
async fn handler_animations_list() -> impl IntoResponse {
    let animations = Storage::list::<Animation>().unwrap();
    Json(animations)
}
