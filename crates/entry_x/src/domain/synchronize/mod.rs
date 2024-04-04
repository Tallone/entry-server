use axum::Router;
use axum::routing::{get, patch, post};
use crate::state::AppState;

mod api;
mod service;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(api::create).patch(api::update).get(api::current))
}