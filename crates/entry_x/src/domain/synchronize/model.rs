use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct SaveReq {
    pub content: String
}