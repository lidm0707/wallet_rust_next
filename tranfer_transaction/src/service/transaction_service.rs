use std::sync::Arc;

use anyhow::Result;
use tracing::info;

use crate::domain::{
    models::transaction_models::SendMoneyModel, repo::transaction_repo::Intotrans,
};

pub struct SendMovieService<T>
where
    T: Intotrans + Send + Sync,
{
    user_repo: Arc<T>,
}

impl<T> SendMovieService<T>
where
    T: Intotrans + Send + Sync,
{
    pub fn new(user_repo: Arc<T>) -> Self {
        Self { user_repo }
    }

    pub async fn send_money_service(&self, send_money_model: SendMoneyModel) -> Result<()> {
        let  send_money_entity = send_money_model.to_entity();
        self.user_repo.send_money(send_money_entity).await?;
        info!("send success");
        Ok(())
    }
}
