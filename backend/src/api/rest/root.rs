//! This file provides general routes and handlers for the REST API.

use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Serialize;

use crate::api::AppState;

pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/", get(handle_health_check))
}

/// Structure of health check response.
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
#[derive(Serialize)]
pub struct Version {
    /// Semantic version of the current build (taken from cargo.tml)
    semver: String,
    /// Current major version of the REST API
    current: String,
    /// Latest available major version of the REST API.
    latest: String,
}

/// Health check handler.
///
/// Delivers information on the current API state.
async fn handle_health_check() -> impl IntoResponse {
    Json(Version {
        semver: env!("CARGO_PKG_VERSION").to_string(),
        current: "v1".to_string(),
        latest: "v1".to_string(),
    })
}
