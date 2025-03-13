use std::sync::Arc;

use anyhow::Result;
use tracing::info;

use crate::{
    backend::argon2_hashing::hash,
    domain::{models::resgis_model::RegisModel, repo::regis_repo::IntoRegis},
};
pub struct RegisService<T>
where
    T: IntoRegis + Send + Sync,
{
    user_repo: Arc<T>,
}

impl<T> RegisService<T>
where
    T: IntoRegis + Send + Sync,
{
    pub fn new(user_repo: Arc<T>) -> Self {
        Self { user_repo }
    }

    pub async fn registor_service(&self, user_registor_model: RegisModel) -> Result<()> {
        let mut user_entity = user_registor_model.to_entity();
        let hashed_password = hash(user_entity.password_hash.clone()).unwrap();
        user_entity.password_hash = hashed_password;
        self.user_repo.registor(user_entity).await?;
        info!("registor success");
        Ok(())
    }
}
