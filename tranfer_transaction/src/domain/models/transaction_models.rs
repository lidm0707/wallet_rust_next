use serde::{Deserialize, Serialize};

use crate::domain::entity::transaction_entitys::SendMoneyEntity;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendMoneyModel {
    pub receiver_id: i32,
    pub amount: f64,
}

impl SendMoneyModel {
    pub fn to_entity(&self,sender_id: i32) -> SendMoneyEntity {
        SendMoneyEntity {
            sender: sender_id,
            receiver: self.receiver_id,
            amount: self.amount,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
