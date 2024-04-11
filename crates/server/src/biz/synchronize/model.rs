use serde::Deserialize;

#[derive(Deserialize)]
pub struct SaveReq {
  pub content: String,
}
