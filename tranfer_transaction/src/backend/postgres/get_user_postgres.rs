use crate::domain::{entity::authen_entity::AuthenEntity, repo::get_user_repo::IntoGetUser};

use super::{conn_postgres::PgPool, schema::users};

use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc; // Assuming `trans` is your table

pub struct UserPostgres {
    db_pool: Arc<PgPool>, // Diesel Async Connection Pool
}

impl UserPostgres {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl IntoGetUser for UserPostgres {
    async fn get_all_user(&self) -> Result<Vec<AuthenEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = users::table
            .select(AuthenEntity::as_select())
            .load::<AuthenEntity>(&mut conn)?;

        Ok(result)
    }


}
