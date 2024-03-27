use crate::{cons, db::DB, domain::entity::users, error::AppError};
use axum::{
  extract::{FromRequestParts, State},
  RequestPartsExt,
};

pub struct LoginUser(users::Model);

/// Extract `users::Model` from token
///
/// When use this extractor, client should bring `cons::HEADER_TOKEN` in request,
/// if token is null or not in cache will return `AppError::LoginRequired`
impl LoginUser {}
// impl FromRequestParts<DB> for LoginUser {
//   type Rejection = AppError;
//
//   async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &DB) -> Result<Self, Self::Rejection> {
//     if let Some(token) = parts.headers.get(cons::HEADER_TOKEN) {
//       let cache = util::cache::redis().await;
//     }
//
//     Err(AppError::LoginRequired)
//   }
// }
