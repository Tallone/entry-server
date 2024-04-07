use axum::{routing::get, Router};

use crate::state::AppState;

mod api;
mod cons;
mod model;
pub(crate) mod service;

pub fn router() -> Router<AppState> {
  Router::new().route("/:license", get(api::check).post(api::active))
}
