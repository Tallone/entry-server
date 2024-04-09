use axum::{extract::State, Json};

use crate::{
  biz::entity::synchronize,
  internal::{db::DB, error::AppError},
  middleware::{authenticator::LoginedUser, response_wrapper::ApiResponse},
};

use super::{model::SaveReq, service};

type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

pub async fn save(user: LoginedUser, State(db): State<DB>, Json(payload): Json<SaveReq>) -> Result<synchronize::Model> {
  let ret = service::Mutation::save(&db.conn, user.0.id, payload.content).await?;
  ApiResponse::ok(ret)
}

pub async fn clear(user: LoginedUser, State(db): State<DB>) -> Result<bool> {
  let ret = service::Mutation::clear(&db.conn, user.0.id).await?;

  ApiResponse::ok(ret)
}

/// Get user's current data, it will create if there have no one
pub async fn current(user: LoginedUser, State(db): State<DB>) -> Result<synchronize::Model> {
  match service::Query::get_by_id(&db.conn, user.0.id).await? {
    Some(v) => ApiResponse::ok(v),
    None => {
      let ret = service::Mutation::save(&db.conn, user.0.id, String::default()).await?;
      ApiResponse::ok(ret)
    }
  }
}
