use serde::Deserialize;

#[derive(Deserialize)]
pub struct ActiveReq {
  pub device_id: Option<String>,
}
