use std::{env, time::Duration};

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use pingora_memory_cache::MemoryCache;
use reqwest::{
  header::{ACCEPT, USER_AGENT},
  Url,
};
use serde_json::Value;

use crate::{consts, AuthUser, GetAccessTokenRequest, OAuthStrategy, Result, DEFAULT_USER_AGENT};

const OAUTH_HOST: &str = "https://github.com";
const API_HOST: &str = "https://api.github.com";

pub struct GithubOAuthStrategy {
  client_id: String,
  client_secret: String,
  redirect_url: String,
  // TODO: Using redis to check state
  cache: MemoryCache<String, ()>,
  token: Option<String>,
}

impl GithubOAuthStrategy {
  pub fn new() -> anyhow::Result<Self> {
    let client_id = env::var(consts::ENV_GITHUB_CLIENT_ID)
      .with_context(|| format!("Missing environment {}", consts::ENV_GITHUB_CLIENT_ID))?;
    let client_secret = env::var(consts::ENV_GITHUB_CLIENT_SECRET)
      .with_context(|| format!("Missing environment {}", consts::ENV_GITHUB_CLIENT_SECRET))?;
    let redirect_url = env::var(consts::ENV_REDIRECT_URL)
      .with_context(|| format!("Missing environment {}", consts::ENV_REDIRECT_URL))?;

    Ok(Self {
      client_id,
      client_secret,
      redirect_url,
      cache: MemoryCache::new(100),
      token: Default::default(),
    })
  }
}

#[async_trait]
impl OAuthStrategy for GithubOAuthStrategy {
  fn get_auth_url(&self) -> Result<Url> {
    let api = format!("{}{}", OAUTH_HOST, "/login/oauth/authorize");
    let mut url: Url = Url::parse(&api).unwrap();

    let state = util::rand_uuid();
    self.cache.put(&state, (), Some(Duration::from_secs(30 * 60)));

    url
      .query_pairs_mut()
      .append_pair("client_id", &self.client_id)
      .append_pair("redirect_url", &self.redirect_url)
      .append_pair("scope", "user")
      .append_pair("state", &state);

    Ok(url)
  }

  async fn get_access_token(&mut self, code: String, state: String) -> Result<String> {
    if self.cache.get(&state).0.is_none() {
      return Err(anyhow!("Invalid state: {state}"));
    }
    let api = format!("{}{}", OAUTH_HOST, "/login/oauth/access_token");
    let body = GetAccessTokenRequest {
      client_id: &self.client_id,
      client_secret: &self.client_secret,
      redirect_uri: &self.redirect_url,
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
      self.token = ac.as_str().map(|s| s.to_owned());
      return self.token.clone().ok_or(anyhow!(
        "Extract access_token failed, maybe Github change the field name"
      ));
    }

    Err(anyhow!("Github not response access_token: {}", response.to_string()))
  }

  async fn get_user(&self) -> Result<AuthUser> {
    let token = self.token.as_ref().ok_or(anyhow!("No access_token"))?;
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

  fn init() -> GithubOAuthStrategy {
    let _ = env_logger::builder()
      .filter_level(log::LevelFilter::Info)
      .is_test(true)
      .try_init();
    dotenv().unwrap();
    GithubOAuthStrategy::new().unwrap()
  }

  #[test]
  fn test_auth_url() {
    let o = init();
    let auth_url = o.get_auth_url().unwrap();
    info!("auth url: {}", auth_url);
  }

  #[tokio::test]
  async fn test_get_info() {
    let code = "182e45dde89fdfad7140";
    let state = "1e0933e9-854a-4568-868f-01c2e5916f5e";
    let mut o = init();
    let token = o.get_access_token(code.to_owned(), state.to_owned()).await.unwrap();
    info!("token: {token}");
    let user = o.get_user().await.unwrap();
    info!("User: {:?}", user);
  }

  #[tokio::test]
  async fn test_redis() {
    let o = init();
    let cli = util::cache::redis().await;
  }
}
