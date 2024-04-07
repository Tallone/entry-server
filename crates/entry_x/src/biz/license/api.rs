use std::{net::SocketAddr, time::Duration};

use axum::{
  extract::{ConnectInfo, Path, State},
  Json,
};
use sea_orm::{Set, TransactionTrait};

use crate::{
  biz::{
    activation,
    entity::{activations, licenses},
  },
  internal::{db::DB, error::AppError},
  middleware::{authenticator::LoginedUser, response_wrapper::ApiResponse},
};

use super::{model::ActiveReq, service};

type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

/// This api will return current user and special `license` state
///
/// Client can used to verify license is still valid
pub async fn check(user: LoginedUser, State(db): State<DB>, Path(license): Path<String>) -> Result<licenses::Model> {
  // check license is exist
  let record = service::Query::get(&db.conn, licenses::Column::Key, &license)
    .await?
    .ok_or(AppError::ResourceNotExist)?;

  // check license is actived by this user
  activation::service::Query::get_by_uid_lk(&db.conn, user.0.id, &license)
    .await?
    .ok_or(AppError::LicenseNotValid)?;
  ApiResponse::ok(record)
}

/// Using a license to active.
///
/// The license must not be used
pub async fn active(
  user: LoginedUser,
  State(db): State<DB>,
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
  Path(license): Path<String>,
  Json(payload): Json<ActiveReq>,
) -> Result<licenses::Model> {
  // check license is exist
  let record = service::Query::get(&db.conn, licenses::Column::Key, &license)
    .await?
    .ok_or(AppError::ResourceNotExist)?;
  if record.is_used {
    return Err(AppError::LicenseNotValid);
  }

  // active this license and save activation
  let txn = db.conn.begin().await?;

  let expired_at = util::current_ms() + Duration::from_secs(record.duration as u64).as_millis() as u64;

  let ret: licenses::Model = service::Mutation::update(
    &txn,
    licenses::ActiveModel {
      id: Set(record.id),
      is_used: Set(true),
      expired_at: Set(Some(expired_at as i64)),
      ..Default::default()
    },
  )
  .await?;

  activation::service::Mutation::create(
    &txn,
    activations::ActiveModel {
      license_key: Set(ret.key.clone()),
      user_id: Set(user.0.id),
      device_id: Set(payload.device_id),
      ip_address: Set(addr.to_string()),
      ..Default::default()
    },
  )
  .await?;

  txn.commit().await?;
  ApiResponse::ok(ret)
}
