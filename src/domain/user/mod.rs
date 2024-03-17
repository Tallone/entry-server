use axum::{routing::get, Router};

mod api;
mod model;
pub mod service;

pub fn router() -> Router {
  Router::new().route("/", get(api::index))
}
