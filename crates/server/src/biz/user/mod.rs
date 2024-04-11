use axum::{
  http::Method,
  routing::{get, patch, post},
};

use crate::internal::router_tree::RouteNode;

mod api;
mod auth_api;
pub mod cons;
mod model;
pub mod service;

// .route("/password", patch(api::update_password))
// .nest(
// "/oauth",
// Router::new().route("/:provider", get(auth_api::oauth_url).post(auth_api::oauth_login)),
// )
pub fn apis() -> RouteNode {
  let mut parent = RouteNode::new("");
  parent
    .path("/")
    .handler(Method::GET, get(api::current))
    .handler(Method::POST, post(api::create));
  parent.path("/login").handler(Method::POST, post(api::login));
  parent
    .path("/password")
    .handler(Method::PATCH, patch(api::update_password));

  let mut oauth_apis = RouteNode::new("");
  oauth_apis
    .path("/:provider")
    .handler(Method::GET, get(auth_api::oauth_url))
    .handler(Method::POST, post(auth_api::oauth_login));
  parent.nest("/oauth", oauth_apis);
  parent
}
