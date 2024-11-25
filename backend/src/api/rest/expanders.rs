//! This file provides general routes and handlers for CRUD operations regarding `Expander`s specifically.

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use crate::api::AppState;
use crate::hardware::Expander;

/// Consolidates all available REST API routes for `Expander`.
pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/", get(handler_expanders_list)) //.post(handler_create_expander))
}

/// GET /:version/expanders.
/// Retrieves all expanders information.
async fn handler_expanders_list(State(state): State<AppState>) -> impl IntoResponse {
    let expanders = state.database.read().list::<Expander>().unwrap();
    // let payloads = expanders
    //     .into_iter()
    //     .map(|(id, expander)| (id, ExpanderPayload::from(expander)))
    //     .collect::<HashMap<Id, ExpanderPayload>>();
    Json(expanders)
}

// /// POST /:version/hardwares.
// /// Create a new expander.
// async fn handler_create_expander(
//     State(state): State<AppState>,
//     Json(payload): Json<CreateExpander>,
// ) -> impl IntoResponse {
//     let expander: Expander = payload.into();
//     let expander = state.database.write().insert(expander).unwrap();
//     Json(expander)
// }
