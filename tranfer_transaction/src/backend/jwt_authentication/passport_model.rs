use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct Passport {
    pub access_token: String,
    pub refresh_token: String,
}
