pub mod consts;
pub mod github;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthUser {
  pub name: String,
  pub email: String,
  pub avatar: String,
}

pub trait OAuthStrategy {
  async fn get_auth_url(&self) -> anyhow::Result<String>;

  async fn exhcange_code(&self, code: String) -> anyhow::Result<String>;

  async fn get_user(&self) -> anyhow::Result<AuthUser>;
}
