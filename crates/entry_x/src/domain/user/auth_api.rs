use std::str::FromStr;

use axum::extract::Path;
use oauth_client::{consts::OAuthProvider, get_strategy, OAuthStrategy};

use crate::{error::AppError, middleware::response_wrapper::ApiResponse};

type Result<T> = std::result::Result<ApiResponse<T>, AppError>;

pub(crate) async fn oauth_url(Path(provider): Path<String>) -> Result<String> {
  let provider = OAuthProvider::from_str(&provider)?;
  let strategy = get_strategy(provider);
  let url = strategy.get_auth_url().await?;
  Ok(ApiResponse::ok(url.to_string()))
}
