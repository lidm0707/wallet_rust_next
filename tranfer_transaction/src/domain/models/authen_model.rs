use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthenModel {
    pub username: String,
    pub password: String,
}
