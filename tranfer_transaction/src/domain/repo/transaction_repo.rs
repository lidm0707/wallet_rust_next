use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::entity::transaction_entitys::SendMoneyEntity;

#[automock]
#[async_trait]
pub trait Intotrans {
    async fn send_money(&self, send_money_entity: SendMoneyEntity) -> Result<()>;
}
