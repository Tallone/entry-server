use std::str::FromStr;

use thiserror::Error;

pub const ENV_GITHUB_CLIENT_ID: &str = "ENTRY_OAUTH_GITHUB_CLIENT_ID";
pub const ENV_GITHUB_CLIENT_SECRET: &str = "ENTRY_OAUTH_GITHUB_CLIENT_SECRET";

#[derive(Error, Debug)]
pub enum OAuthError {
  #[error("Not support oauth provider")]
  NotSupport,

  #[error("Invalid oauth state")]
  InvalidState,

  #[error("Http error")]
  Reqwest(#[from] reqwest::Error),

  #[error(transparent)]
  Other(#[from] anyhow::Error),
}

pub enum OAuthProvider {
  Github,
}

impl FromStr for OAuthProvider {
  type Err = OAuthError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "github" => Ok(Self::Github),
      _ => Err(OAuthError::NotSupport),
    }
  }
}
