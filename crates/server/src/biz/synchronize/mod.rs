use crate::internal::router_tree::RouteNode;
use axum::http::Method;
use axum::routing::{delete, get, post};

mod api;
mod model;
mod service;

pub fn apis() -> RouteNode {
  let mut node = RouteNode::new("/");
  node
    .handler(Method::GET, get(api::current))
    .handler(Method::POST, post(api::save))
    .handler(Method::DELETE, delete(api::clear));
  node
}
