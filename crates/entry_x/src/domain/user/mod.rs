use axum::{
  routing::{get, post},
  Router,
};

use crate::state::AppState;

mod api;
mod auth_api;
mod model;
pub mod service;

pub fn router() -> Router<AppState> {
  Router::new()
    .route("/", post(api::create))
    .route("/:id", get(api::get))
    .nest("/oauth", Router::new().route("/:provider", get(auth_api::oauth_url)))
}
