use std::io;

use oauth_client::consts::OAuthError;
use thiserror::Error;
use util::RedisError;

pub const MSG_INTERNAL_ERROR: &str = "Internal error";

#[derive(Error, Debug)]
pub enum AppError {
  #[error("Unknown error: {0}")]
  Unknown(String),

  #[error("IO Error: {0}")]
  IO(#[from] io::Error),

  #[error("DB Error: {0}")]
  Db(#[from] sea_orm::DbErr),

  #[error("Redis Error: {0}")]
  Redis(#[from] RedisError),

  #[error(transparent)]
  Biz(#[from] anyhow::Error),

  #[error("Api is not found")]
  ApiNotFound,

  #[error("Request is not valid: {0}")]
  RequestNotValid(String),

  #[error("Resource not exist")]
  ResourceNotExist,

  #[error("The provided client token is invalid or expired. Please check your credentials and try again.")]
  InvalidToken,

  #[error("This user is in deactive state.")]
  DeactivedUser,

  #[error("This email is already in used.")]
  EmailExist,

  #[error(transparent)]
  OAuth(#[from] OAuthError),

  #[error("This license is not valid")]
  LicenseNotValid,
}

impl AppError {
  pub fn code(&self) -> u32 {
    match self {
      AppError::Unknown(_) => 9999,
      AppError::Biz(_) => 8000,
      AppError::IO(_) => 1500,
      AppError::Db(_) => 1510,
      AppError::Redis(_) => 1520,
      AppError::RequestNotValid(_) => 4010,
      AppError::OAuth(_) => 4011,
      AppError::ResourceNotExist => 4040,
      AppError::InvalidToken => 4403,
      AppError::ApiNotFound => 4404,
      AppError::LicenseNotValid => 4300,
      AppError::DeactivedUser => 4600,
      AppError::EmailExist => 4601,
    }
  }

  pub fn message(&self) -> String {
    match self {
      AppError::IO(_) | AppError::Db(_) | AppError::Redis(_) => MSG_INTERNAL_ERROR.to_string(),
      _ => self.to_string(),
    }
  }
}
