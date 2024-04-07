use serde::Deserialize;

#[derive(Deserialize)]
pub struct ActiveReq {
  pub license_key: String,
  pub device_id: Option<String>,
}
