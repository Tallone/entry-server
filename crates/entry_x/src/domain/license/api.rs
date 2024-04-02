use axum::extract::{Path, State};

use crate::{
  db::DB,
  domain::{activation, entity::licenses},
  error::AppError,
  middleware::{authenticator::LoginedUser, response_wrapper::ApiResponse},
};

use super::service;

type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

/// This api will return current user and special `license` status
///
/// Client can used to verify license is still valid
pub async fn check(user: LoginedUser, State(db): State<DB>, Path(license): Path<String>) -> Result<licenses::Model> {
  // check license is exist
  let record = service::Query::get(&db.conn, licenses::Column::Key, &license)
    .await?
    .ok_or(AppError::ResourceNotExist)?;

  // check license is actived by this user
  activation::service::Query::get_by_uid_lid(&db.conn, user.0.id, &license)
    .await?
    .ok_or(AppError::LicenseNotValid)?;
  Ok(ApiResponse::ok(record))
}

pub async fn active(State(db): State<DB>) {}
