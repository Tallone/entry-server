use axum::{extract::State, Json};

use crate::{
  db::DB,
  domain::entity::synchronize,
  error::AppError,
  middleware::{authenticator::LoginedUser, response_wrapper::ApiResponse},
};

use super::{model::SaveReq, service};

type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

pub async fn save(user: LoginedUser, State(db): State<DB>, Json(payload): Json<SaveReq>) -> Result<synchronize::Model> {
  let ret = service::save(&db.conn, user.0.id, payload.content).await?;
  Ok(ApiResponse::ok(ret))
}

pub async fn clear(user: LoginedUser, State(db): State<DB>) -> Result<bool> {
  let ret = service::clear(&db.conn, user.0.id).await?;

  Ok(ApiResponse::ok(ret))
}

/// Get user's current data, it will create if there have no one
pub async fn current(user: LoginedUser, State(db): State<DB>) -> Result<synchronize::Model> {
  match service::get(&db.conn, user.0.id).await? {
    Some(v) => Ok(ApiResponse::ok(v)),
    None => {
      let ret = service::save(&db.conn, user.0.id, String::default()).await?;
      Ok(ApiResponse::ok(ret))
    }
  }
}
