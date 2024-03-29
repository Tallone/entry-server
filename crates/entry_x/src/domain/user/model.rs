use serde::Deserialize;
use uuid::Uuid;

/// Create user request
#[derive(Deserialize)]
pub(crate) struct CreateReq {
  // User email
  pub email: String,
  // User plaintext password
  pub password: String,
  // User name
  pub name: Option<String>,
}

/// Get one user request.
///
/// Below is an json that represent [`GetOneOptions::Email`]:
///
/// ```
/// {
///   "email": "test@entry.com"
/// }
/// ```
#[derive(Deserialize)]
pub(crate) enum GetReq {
  Id(Uuid),
  Email(String),
}

#[derive(Deserialize)]
pub(crate) struct OAuthLoginReq {
  pub state: String,
  pub code: String,
}

#[derive(Deserialize)]
pub(crate) struct LoginReq {
  pub email: String,
  pub password: String,
}
