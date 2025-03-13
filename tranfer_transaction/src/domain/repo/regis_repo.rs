use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::entity::resgis_entity::RegisEntity;

#[automock]
#[async_trait]
pub trait IntoRegis {
    async fn registor(&self, user_registor_entity: RegisEntity) -> Result<()>;
}
