use axum::{handler::Handler, http::Method, routing::get, Router};

use crate::internal::{app_state::AppState, router_tree::RouteNode};

mod api;
mod model;
pub(crate) mod service;

pub fn router() -> Router<AppState> {
  Router::new().route("/:license", get(api::check).post(api::active))
}

pub fn register_apis<H, T>(parent: &mut RouteNode<H, AppState>)
where
  H: Handler<T, AppState>,
  T: 'static,
{
  parent.path("/:license").handler(api::check, Method::GET);
}
