use std::str::FromStr;

use axum::{
  extract::{Path, State},
  Json,
};
use oauth_client::{consts::OAuthProvider, get_strategy, OAuthStrategy};

use crate::{db::DB, error::AppError, middleware::response_wrapper::ApiResponse};

use super::{
  model::{GetReq, OAuthLoginReq},
  service::{self, Mutation, Query},
};

type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

pub(crate) async fn oauth_url(Path(provider): Path<String>) -> Result<String> {
  let provider = OAuthProvider::from_str(&provider)?;
  let strategy = get_strategy(provider);
  let url = strategy.get_auth_url().await?;
  Ok(ApiResponse::ok(url.to_string()))
}

/// Perform OAuth login with provider code and state.
///
/// If the retrieved email information does not exist, it will register as a new user.
pub(crate) async fn oauth_login(
  Path(provider): Path<String>,
  Json(payload): Json<OAuthLoginReq>,
  State(db): State<DB>,
) -> Result<String> {
  // TODO: Verify payload
  let provider = OAuthProvider::from_str(&provider)?;
  let strategy = get_strategy(provider);
  let access_token = strategy.get_access_token(payload.code, payload.state).await?;
  let auth_user = strategy.get_user(&access_token).await?;
  match Query::get(db.conn, GetReq::Email(auth_user.email)).await? {
    Some(u) => service::get_user(token, id, db),
    None => {}
  }

  Ok("".to_owned())
}
