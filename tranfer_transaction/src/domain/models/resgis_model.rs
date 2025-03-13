use serde::{Deserialize, Serialize};

use crate::domain::entity::resgis_entity::RegisEntity;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisModel {
    pub username: String,
    pub password: String,
}

impl RegisModel {
    pub fn to_entity(&self) -> RegisEntity {
        RegisEntity {
            username: self.username.clone(),
            password_hash: self.password.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            amount: 0.00,
        }
    }
}

// id -> Int4,
// #[max_length = 255]
// username -> Varchar,
// #[max_length = 255]
// password_hash -> Varchar,
// created_at -> Timestamp,
// updated_at -> Timestamp,
// amount -> Float8,
