pub mod consts;
pub mod github;

use async_trait::async_trait;
use reqwest::Url;
use serde::{Deserialize, Serialize};

pub(crate) type Result<T> = anyhow::Result<T>;
pub(crate) const DEFAULT_USER_AGENT: &str = "Entry-X";

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
  pub name: String,
  pub email: String,
  #[serde(alias = "avatar_url")]
  pub avatar: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GetAccessTokenRequest<'a> {
  pub client_id: &'a str,
  pub client_secret: &'a str,
  pub redirect_uri: &'a str,
  pub code: String,
}

#[async_trait]
pub trait OAuthStrategy {
  fn get_auth_url(&self) -> Result<Url>;

  async fn get_access_token(&mut self, code: String, state: String) -> Result<String>;

  async fn get_user(&self) -> Result<AuthUser>;
}
