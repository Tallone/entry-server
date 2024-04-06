use axum::{routing::get, Router};

use crate::state::AppState;

mod api;
pub(crate) mod service;

pub fn router() -> Router<AppState> {
  Router::new().route("/", get(api::check))
}
