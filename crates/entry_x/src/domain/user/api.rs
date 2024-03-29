use anyhow::anyhow;
use axum::{extract::State, Json};
use sea_orm::ActiveValue::*;

use crate::{
  db::DB,
  domain::entity::users,
  error::AppError,
  middleware::{authenticator::LoginedUser, response_wrapper::ApiResponse},
};

use super::{
  model::{CreateReq, GetReq, LoginReq},
  service::{self, Mutation},
};

type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

/// Create a new user with giving `payload`
pub async fn create(State(db): State<DB>, Json(payload): Json<CreateReq>) -> Result<users::Model> {
  let passwd = util::argon2_encrypt(&payload.password)?;
  let act_model = users::ActiveModel {
    email: Set(payload.email),
    password: Set(Some(passwd)),
    name: match payload.name {
      Some(v) => Set(Some(v)),
      None => NotSet,
    },
    ..Default::default()
  };

  let ret = Mutation::create(&db.conn, act_model).await?;
  Ok(ApiResponse::ok(ret))
}

/// Logging in with password and return token
pub async fn login(State(db): State<DB>, Json(payload): Json<LoginReq>) -> Result<String> {
  if let Some(db) = service::Query::get(&db.conn, GetReq::Email(payload.email)).await? {
    // Logging in with a password is forbidden when not set password
    let hashed = db.password.ok_or(anyhow!("This user can't login using password"))?;

    // check password
    if util::argon2_verify(&payload.password, &hashed).is_err() {
      return Ok(ApiResponse::failed(anyhow!("Password is not matched").into()));
    }

    // login success, return token
    let token = service::create_token(&db.id.to_string()).await?;
    return Ok(ApiResponse::ok(token));
  }

  Ok(ApiResponse::failed(AppError::ResourceNotExist))
}

/// Get current login user info
pub async fn current(user: LoginedUser) -> Result<users::Model> {
  Ok(ApiResponse::ok(user.0))
}
