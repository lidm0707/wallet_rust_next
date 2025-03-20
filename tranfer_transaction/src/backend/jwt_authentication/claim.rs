use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct Claim {
    pub id : i32,
    pub user: String,
    pub exp: usize,
    pub iat: usize,
}
