use axum::{routing::get, Router};

use crate::internal::app_state::AppState;

mod api;
mod model;
pub(crate) mod service;

pub fn router() -> Router<AppState> {
  Router::new().route("/:license", get(api::check).post(api::active))
}
