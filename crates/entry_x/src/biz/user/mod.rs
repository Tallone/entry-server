use axum::{
  routing::{get, patch, post},
  Router,
};

use crate::state::AppState;

mod api;
mod auth_api;
pub mod cons;
mod model;
pub mod service;

pub fn router() -> Router<AppState> {
  Router::new()
    .route("/", post(api::create).get(api::current))
    .route("/login", post(api::login))
    .route("/password", patch(api::update_password))
    .nest(
      "/oauth",
      Router::new().route("/:provider", get(auth_api::oauth_url).post(auth_api::oauth_login)),
    )
}
