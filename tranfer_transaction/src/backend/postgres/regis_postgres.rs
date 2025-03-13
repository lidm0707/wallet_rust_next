use crate::domain::{entity::resgis_entity::RegisEntity, repo::regis_repo::IntoRegis};

use super::{conn_postgres::PgPool, schema::users};

use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc; // Assuming `trans` is your table

pub struct RegisPostgres {
    db_pool: Arc<PgPool>, // Diesel Async Connection Pool
}

impl RegisPostgres {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl IntoRegis for RegisPostgres {
    async fn registor(&self, user_register_entity: RegisEntity) -> Result<()> {
        let mut user_register_entity = user_register_entity;
        user_register_entity.amount = 100.00;
        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::insert_into(users::table)
            .values(&user_register_entity)
            .execute(&mut conn)?;

        Ok(())
    }
}
