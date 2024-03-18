use axum::{
  routing::{get, post},
  Router,
};

use crate::state::AppState;

mod api;
mod model;
pub mod service;

pub fn router() -> Router<AppState> {
  Router::new().route("/", get(api::index)).route("/", post(api::create))
}
