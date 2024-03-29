use std::io;

use oauth_client::consts::OAuthError;
use thiserror::Error;
use util::RedisError;

pub const MSG_INTERNAL_ERROR: &str = "Internal error";

#[derive(Error, Debug)]
pub enum AppError {
  #[error(transparent)]
  Biz(#[from] anyhow::Error),

  #[error("Api is not found")]
  ApiNotFound,

  #[error("IO Error: {0}")]
  IO(#[from] io::Error),

  #[error("DB Error: {0}")]
  Db(#[from] sea_orm::DbErr),

  #[error("Redis Error: {0}")]
  Redis(#[from] RedisError),

  #[error("Unhandled internal error")]
  Unknown,

  #[error("Request is not valid")]
  RequestNotValid,

  #[error("Resource not exist")]
  ResourceNotExist,

  #[error("The provided client token is invalid or expired. Please check your credentials and try again.")]
  InvalidToken,

  #[error(transparent)]
  OAuth(#[from] OAuthError),
}

impl AppError {
  pub fn code(&self) -> u32 {
    match self {
      AppError::Unknown => 9999,
      AppError::Biz(_) => 8000,
      AppError::IO(_) => 1500,
      AppError::Db(_) => 1510,
      AppError::Redis(_) => 1520,
      AppError::RequestNotValid => 4010,
      AppError::OAuth(_) => 4011,
      AppError::ResourceNotExist => 4040,
      AppError::InvalidToken => 4403,
      AppError::ApiNotFound => 4404,
    }
  }

  pub fn message(&self) -> String {
    match self {
      AppError::IO(_) | AppError::Db(_) | AppError::Redis(_) => MSG_INTERNAL_ERROR.to_string(),
      _ => self.to_string(),
    }
  }
}
