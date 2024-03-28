use std::str::FromStr;

use axum::{
  extract::{Path, State},
  Json,
};
use axum_macros::debug_handler;
use oauth_client::{consts::OAuthProvider, get_strategy, OAuthStrategy};
use sea_orm::Set;

use crate::{db::DB, domain::entity::users, error::AppError, middleware::response_wrapper::ApiResponse};

use super::{
  cons::UserState,
  model::{GetReq, OAuthLoginReq},
  service::{self, Query},
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
  State(db): State<DB>,
  Json(payload): Json<OAuthLoginReq>,
) -> Result<String> {
  // TODO: Verify payload
  let provider = OAuthProvider::from_str(&provider)?;
  let strategy = get_strategy(provider);
  let access_token = strategy.get_access_token(payload.code, payload.state).await?;
  let auth_user = strategy.get_user(&access_token).await?;
  let record = match Query::get(&db.conn, GetReq::Email(auth_user.email.clone())).await? {
    Some(u) => u,
    None => {
      let u = service::Mutation::create(
        &db.conn,
        users::ActiveModel {
          email: Set(auth_user.email),
          status: Set(UserState::Active as i16),
          name: Set(Some(auth_user.name)),
          ..Default::default()
        },
      )
      .await?;
      u
    }
  };

  let token = util::rand_str(64);
  service::create_token(&token, &record.id.to_string()).await?;

  Ok(ApiResponse::ok(token))
}
