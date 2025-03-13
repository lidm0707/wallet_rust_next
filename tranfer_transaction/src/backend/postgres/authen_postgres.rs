use crate::domain::{entity::authen_entity::AuthenEntity, repo::authen_repo::IntoAuthen};

use super::{conn_postgres::PgPool, schema::users};

use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc; // Assuming `trans` is your table

pub struct AuthenPostgres {
    db_pool: Arc<PgPool>, // Diesel Async Connection Pool
}

impl AuthenPostgres {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl IntoAuthen for AuthenPostgres {
    async fn find_by_username(&self, username: &str) -> Result<AuthenEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = users::table
            .filter(users::username.eq(username))
            .select(AuthenEntity::as_select())
            .first::<AuthenEntity>(&mut conn)?;

        Ok(result)
    }


}
