use axum::routing::post;
use axum::Router;

use crate::internal::app_state::AppState;

mod api;
mod model;
mod service;

pub fn router() -> Router<AppState> {
  Router::new().route("/", post(api::save).get(api::current).delete(api::clear))
}
