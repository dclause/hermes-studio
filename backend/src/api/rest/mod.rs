//! This module contains all the code related to the REST events handling.

use axum::Router;
use serde::Deserialize;

use crate::api::AppState;

mod config;
mod root;

/// Generic pagination query parameters to be reused when needed across endpoints.
/// Ex. {offset: 3, limit: 3} will return 3 elements, number 3 to 5: [1,2,X,X,X,6,7,...]
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    /// Number of elements to skip in the response.
    pub offset: Option<usize>,
    /// Number of elements in the response.
    pub limit: Option<usize>,
}

/// Consolidates all available REST API routes.
pub(crate) fn build_rest_routes() -> Router<AppState> {
    Router::new()
        .nest("/", root::routes())
        .nest("/config", config::routes())
    // .nest("/boards", boards::routes())
    // .nest("/animations", animations::routes())
}
