pub mod consts;
pub mod github;

use std::sync::Arc;

use async_trait::async_trait;
use consts::{OAuthError, OAuthProvider};
use github::GithubOAuthStrategy;
use reqwest::Url;
use serde::{Deserialize, Serialize};

pub(crate) type Result<T> = std::result::Result<T, OAuthError>;
pub(crate) const DEFAULT_USER_AGENT: &str = "Entry-X";
const CACHE_OAUTH_STATE_PREFIX: &str = "oauth_state_";

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
  pub code: &'a str,
}

#[async_trait]
pub trait OAuthStrategy {
  async fn get_auth_url(&self, redirect_url: &str) -> Result<Url>;

  async fn get_access_token(&self, code: &str, state: &str) -> Result<String>;

  async fn get_user(&self, token: &str) -> Result<AuthUser>;

  fn get_state_cache_key(&self, state: &str) -> String {
    format!("{}{}", CACHE_OAUTH_STATE_PREFIX, state)
  }
}

pub fn get_strategy(provider: OAuthProvider) -> Arc<&'static impl OAuthStrategy> {
  match provider {
    OAuthProvider::Github => Arc::new(GithubOAuthStrategy::new()),
  }
}
