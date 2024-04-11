use crate::internal::{error::AppError, router_tree::RouteNode};
pub(crate) mod activation;
pub(crate) mod entity;
pub(crate) mod license;
pub(crate) mod macros;
pub(crate) mod synchronize;
pub(crate) mod user;

pub(crate) type Result<T> = std::result::Result<T, AppError>;

pub fn init(root: &mut RouteNode) {
  root
    .nest("/v1/license", license::apis())
    .nest("/v1/user", user::apis())
    .nest("/v1/sync", synchronize::apis());
}
