use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::entity::authen_entity::AuthenEntity;

#[automock]
#[async_trait]
pub trait IntoAuthen {
    async fn find_by_username(&self, username: &str) -> Result<AuthenEntity>;
}
