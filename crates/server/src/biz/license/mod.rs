use axum::{
  http::Method,
  routing::{get, post},
};

use crate::internal::router_tree::RouteNode;

mod api;
mod model;
pub(crate) mod service;

pub fn apis() -> RouteNode {
  let mut node = RouteNode::new("/:license");
  node
    .handler(Method::GET, get(api::check))
    .handler(Method::POST, post(api::active));

  node
}
