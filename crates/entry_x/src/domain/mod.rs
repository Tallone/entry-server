use axum::Router;

use crate::state::AppState;
pub(crate) mod entity;
pub(crate) mod user;

pub fn router() -> Router<AppState> {
  Router::new().nest("/v1/user", user::router())
}
