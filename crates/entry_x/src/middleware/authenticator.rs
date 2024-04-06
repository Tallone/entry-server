use crate::{
  cons,
  db::DB,
  biz::{entity::users, user},
  error::AppError,
};
use axum::{
  async_trait,
  extract::{FromRef, FromRequestParts},
};

pub struct LoginedUser(pub users::Model);

/// Extract `users::Model` from token
///
/// When use this extractor, client should bring `cons::HEADER_TOKEN` in request,
/// if token is null or not in cache will return `AppError::LoginRequired`
#[async_trait]
impl<S> FromRequestParts<S> for LoginedUser
where
  DB: FromRef<S>,
  S: Send + Sync,
{
  type Rejection = AppError;

  async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
    if let Some(token) = parts.headers.get(cons::HEADER_TOKEN) {
      let db = DB::from_ref(state);
      let token = token.to_str().map_err(|_| AppError::RequestNotValid)?;
      let u = user::service::get_user_by_token(token, &db.conn)
        .await?
        .ok_or(AppError::InvalidToken)?;
      return Ok(LoginedUser(u));
    }

    Err(AppError::InvalidToken)
  }
}
