//! This file provides general routes and handlers for CRUD operations regarding frontend specific configurations.

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::Value;

use crate::api::AppState;
use crate::utils::interface::Interface;

/// Consolidates all available REST API routes for `Board`.
pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(get_config).post(set_config))
}

/// GET /:version/config.
/// Retrieves all interface configurations.
async fn get_config(State(state): State<AppState>) -> impl IntoResponse {
    let config = Interface::get_config_from_db(state.database).unwrap();
    Json(config)
}

/// POST /:version/config.
/// Save all interface configurations.
async fn set_config(State(state): State<AppState>, Json(config): Json<Value>) -> impl IntoResponse {
    let config = Interface::set_config_to_db(state.database, config).unwrap();
    state.socket.emit("config:get", &config).unwrap();
    Json(config)
}
