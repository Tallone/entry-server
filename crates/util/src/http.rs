use std::{cell::OnceCell, sync::OnceLock, time::Duration};

use anyhow::anyhow;
use reqwest::{Client, ClientBuilder};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

static INSTANCE: OnceLock<Client> = OnceLock::new();

/// Get a `reqwest` http Client
///
/// System proxies and `HTTP_PROXY`, `HTTPS_PROXY` are enabled
pub fn client() -> &'static Client {
  INSTANCE.get_or_init(|| {
    ClientBuilder::new()
      .connect_timeout(CONNECT_TIMEOUT)
      .timeout(REQUEST_TIMEOUT)
      .build()
      .unwrap()
  })
}
