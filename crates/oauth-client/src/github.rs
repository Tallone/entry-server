use std::{env, str::FromStr, time::Duration};

use anyhow::Context;
use pingora_memory_cache::MemoryCache;
use reqwest::Url;

use crate::{consts, OAuthStrategy};

const OAUTH_HOST: &str = "https://github.com";

pub struct GithubOAuthStrategy {
  client_id: String,
  client_secret: String,
  redirect_url: String,
  cache: MemoryCache<String, ()>,
  token: String,
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

impl OAuthStrategy for GithubOAuthStrategy {
  async fn get_auth_url(&self) -> anyhow::Result<String> {
    let api = format!("{}{}", OAUTH_HOST, "/login/oauth/authorize");
    let mut url: Url = Url::parse(&api).unwrap();
    let mut query = url.query_pairs_mut();
    query.append_pair("client_id", &self.client_id);
    query.append_pair("redirect_url", &self.redirect_url);
    query.append_pair("scope", "user");
    let state = util::rand_uuid();
    self.cache.put(&state, (), Some(Duration::from_secs(30 * 60)));
    query.append_pair("state", &state);

    Ok("".into())
  }

  async fn exhcange_code(&self, code: String) -> anyhow::Result<String> {
    todo!()
  }

  async fn get_user(&self) -> anyhow::Result<crate::AuthUser> {
    todo!()
  }
}
