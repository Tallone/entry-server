use std::str::FromStr;

use axum::extract::State;
use oauth::{consts::OAuthProvider, get_strategy, OAuthStrategy};
use sea_orm::Set;

use crate::{
  biz::entity::users,
  internal::{db::DB, error::AppError},
  middleware::response_wrapper::{ApiResponse, Json, Path, Query},
};

use super::{
  cons::UserState,
  model::{GetReq, OAuthLoginReq, OAuthUrlParams},
  service,
};

type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

/// Get `provider` authentication url
///
/// When auth finished, browser will redirect to `redirect_url`
pub(crate) async fn oauth_url(Path(provider): Path<String>, Query(params): Query<OAuthUrlParams>) -> Result<String> {
  let provider = OAuthProvider::from_str(&provider)?;
  let strategy = get_strategy(provider);
  let url = strategy.get_auth_url(&params.redirect_url).await?;
  ApiResponse::ok(url.to_string())
}

/// Perform OAuth login with `provider` code and state.
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
  let access_token = strategy.get_access_token(&payload.code, &payload.state).await?;
  let auth_user = strategy.get_user(&access_token).await?;
  let record = match service::Query::get_opt(&db.conn, GetReq::Email(auth_user.email.clone())).await? {
    Some(u) => u,
    None => {
      service::Mutation::create(
        &db.conn,
        users::ActiveModel {
          email: Set(auth_user.email),
          state: Set(UserState::Active as i16),
          name: Set(Some(auth_user.name)),
          ..Default::default()
        },
      )
      .await?
    }
  };

  let token = service::create_token(&record.id.to_string()).await?;

  ApiResponse::ok(token)
}
