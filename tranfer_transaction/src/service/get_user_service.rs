use std::sync::Arc;

use anyhow::Result;
use tracing::info;

use crate::domain::{
    models::get_all_user_model::GetAllUserModel,
    repo::get_user_repo::IntoGetUser,
};

pub struct GetUserService<T>
where
    T: IntoGetUser + Send + Sync,
{
    user_repo: Arc<T>,
}

impl<T> GetUserService<T>
where
    T: IntoGetUser + Send + Sync,
{
    pub fn new(user_repo: Arc<T>) -> Self {
        Self { user_repo }
    }

    pub async fn get_all_user(&self) -> Result<Vec<GetAllUserModel>> {
        let raws = self.user_repo.get_all_user().await?;
        let mut result = Vec::<GetAllUserModel>::new();
        for row in raws {
            result.push(GetAllUserModel {
                username: row.username,
                id: row.id,
            });
        }

        info!("get user success");
        Ok(result)
    }
}
