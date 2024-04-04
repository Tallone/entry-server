use crate::state::AppState;
use axum::routing::post;
use axum::Router;

mod api;
mod service;
mod model;

pub fn router() -> Router<AppState> {
  Router::new().route(
    "/",
    post(api::save)
      .get(api::current)
      .delete(api::clear),
  )
}
