//! This file provides general routes and handlers for CRUD operations regarding frontend specific configurations.

use axum::{Json, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use toml::Value;

use crate::api::AppState;
use crate::utils::interface::Interface;

/// Consolidates all available REST API routes for `Board`.
pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(get_config).post(set_config))
}

/// GET /:version/config.
/// Retrieves all interface configurations.
async fn get_config() -> impl IntoResponse {
    let config = Interface::get_config().unwrap();
    Json(config)
}

/// POST /:version/config.
/// Save all interface configurations.
async fn set_config(State(state): State<AppState>, Json(input): Json<Value>) -> impl IntoResponse {
    let config = Interface::set_config(input).unwrap();
    state.socket.emit("config:get", config.clone()).unwrap();
    Json(config)
}
