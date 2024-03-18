use axum::Router;

use crate::{error::AppError, state::AppState};
pub(crate) mod entity;
pub(crate) mod user;
pub(crate) use entity::prelude::*;

pub fn router() -> Router<AppState> {
  Router::new().nest("/v1/user", user::router())
}
