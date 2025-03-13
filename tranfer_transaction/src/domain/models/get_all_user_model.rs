use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAllUserModel {
    pub id: i32,
    pub username: String,
}
