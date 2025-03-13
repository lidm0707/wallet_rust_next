use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::entity::authen_entity::AuthenEntity;

#[automock]
#[async_trait]
pub trait IntoGetUser {
    async fn get_all_user(&self) -> Result<Vec<AuthenEntity>>;
}
