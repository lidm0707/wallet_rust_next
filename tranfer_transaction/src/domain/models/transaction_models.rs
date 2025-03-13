use serde::{Deserialize, Serialize};

use crate::domain::entity::transaction_entitys::SendMoneyEntity;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendMoneyModel {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub amount: f64,
}

impl SendMoneyModel {
    pub fn to_entity(&self) -> SendMoneyEntity {
        SendMoneyEntity {
            sender: self.sender_id,
            receiver: self.receiver_id,
            amount: self.amount,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
