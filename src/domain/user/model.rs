use serde::Deserialize;

/// Create user request
#[derive(Deserialize)]
pub(crate) struct CreateReq {
  // User email
  pub email: String,
  // User password after hash
  pub password: String,
  // Hash salt
  pub hash: String,
  // User name
  pub name: Option<String>,
}
