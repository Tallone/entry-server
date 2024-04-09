use std::{env, sync::OnceLock};

use anyhow::anyhow;
use async_trait::async_trait;
use log::error;
use reqwest::{
  header::{ACCEPT, USER_AGENT},
  Url,
};
use serde_json::Value;
use util::KeysInterface;

use crate::{
  consts::{self, OAuthError},
  AuthUser, GetAccessTokenRequest, OAuthStrategy, Result, DEFAULT_USER_AGENT,
};

const OAUTH_HOST: &str = "https://github.com";
const API_HOST: &str = "https://api.github.com";
static INSTANCE: OnceLock<GithubOAuthStrategy> = OnceLock::new();

pub struct GithubOAuthStrategy {
  client_id: String,
  client_secret: String,
}

impl GithubOAuthStrategy {
  pub fn new() -> &'static Self {
    INSTANCE.get_or_init(|| {
      let client_id = env::var(consts::ENV_GITHUB_CLIENT_ID)
        .unwrap_or_else(|_| panic!("Missing environment {}", consts::ENV_GITHUB_CLIENT_ID));
      let client_secret = env::var(consts::ENV_GITHUB_CLIENT_SECRET)
        .unwrap_or_else(|_| panic!("Missing environment {}", consts::ENV_GITHUB_CLIENT_SECRET));

      Self {
        client_id,
        client_secret,
      }
    })
  }
}

#[async_trait]
impl OAuthStrategy for GithubOAuthStrategy {
  async fn get_auth_url(&self, redirect_url: &str) -> Result<Url> {
    let api = format!("{}{}", OAUTH_HOST, "/login/oauth/authorize");
    let mut url: Url = Url::parse(&api).unwrap();

    let state = util::rand_uuid();
    let cache = util::cache::redis().await;
    cache
      .set(
        self.get_state_cache_key(state.as_str()),
        redirect_url,
        Some(util::Expiration::EX(30 * 60)),
        None,
        false,
      )
      .await
      .map_err(|e| {
        error!("Set cache failed: {}", e);
        OAuthError::InvalidState
      })?;

    url
      .query_pairs_mut()
      .append_pair("client_id", &self.client_id)
      .append_pair("redirect_url", redirect_url)
      .append_pair("scope", "user")
      .append_pair("state", &state);

    Ok(url)
  }

  async fn get_access_token(&self, code: &str, state: &str) -> Result<String> {
    // Check state is in cache
    let redirect_url: String = util::cache::redis()
      .await
      .get(self.get_state_cache_key(state))
      .await
      .map_err(|_| OAuthError::InvalidState)?;

    let api = format!("{}{}", OAUTH_HOST, "/login/oauth/access_token");
    let body = GetAccessTokenRequest {
      client_id: &self.client_id,
      client_secret: &self.client_secret,
      redirect_uri: &redirect_url,
      code,
    };
    let client = util::http::client();
    let response: Value = client
      .post(api)
      .json(&body)
      .header(ACCEPT, "application/json")
      .send()
      .await?
      .json()
      .await?;

    if let Some(ac) = response.get("access_token") {
      let ac = ac.as_str().ok_or(anyhow!(
        "Extract access_token failed, maybe Github change the field name"
      ))?;
      return Ok(ac.to_owned());
    }

    Err(OAuthError::Other(anyhow!(response.to_string(),)))
  }

  async fn get_user(&self, token: &str) -> Result<AuthUser> {
    let api = format!("{}{}", API_HOST, "/user");
    let client = util::http::client();
    let response: AuthUser = client
      .get(api)
      .bearer_auth(token)
      .header(ACCEPT, "application/vnd.github+json")
      .header(USER_AGENT, DEFAULT_USER_AGENT)
      .header("X-GitHub-Api-Version", "2022-11-28")
      .send()
      .await?
      .json()
      .await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  use dotenvy::dotenv;
  use log::info;

  use crate::OAuthStrategy;

  use super::GithubOAuthStrategy;

  fn init() -> &'static GithubOAuthStrategy {
    let _ = env_logger::builder()
      .filter_level(log::LevelFilter::Info)
      .is_test(true)
      .try_init();
    dotenv().unwrap();
    GithubOAuthStrategy::new()
  }

  #[tokio::test]
  async fn test_auth_url() {
    let o = init();
    let redirect_url = "http://localhost:3000";
    let auth_url = o.get_auth_url(redirect_url).await.unwrap();
    info!("auth url: {}", auth_url);
  }

  #[tokio::test]
  async fn test_get_info() {
    let code = "182e45dde89fdfad7140";
    let state = "1e0933e9-854a-4568-868f-01c2e5916f5e";
    let o = init();
    let token = o.get_access_token(code, state).await.unwrap();
    info!("token: {token}");
    let user = o.get_user(&token).await.unwrap();
    info!("User: {:?}", user);
  }
}
