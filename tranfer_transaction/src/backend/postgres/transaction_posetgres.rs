use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::domain::{
    entity::transaction_entitys::SendMoneyEntity, repo::transaction_repo::Intotrans,
};

use super::{conn_postgres::PgPool, schema::trans, schema::users};

pub struct TransPostgres {
    db_pool: Arc<PgPool>, // Diesel Async Connection Pool
}

impl TransPostgres {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl Intotrans for TransPostgres {
    async fn send_money(&self, send_money_entity: SendMoneyEntity) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        conn.transaction(|conn| {
            // Insert the transaction record
            diesel::insert_into(trans::table)
                .values(&send_money_entity)
                .execute(conn)?;

            // Decrease the sender's amount
            diesel::update(users::table.filter(users::id.eq(send_money_entity.sender)))
                .set(users::amount.eq(users::amount - send_money_entity.amount))
                .execute(conn)?;

            // Increase the recipient's amount
            diesel::update(users::table.filter(users::id.eq(send_money_entity.receiver)))
                .set(users::amount.eq(users::amount + send_money_entity.amount))
                .execute(conn)?;

            Ok(())
        })
    }
}
