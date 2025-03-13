use crate::backend::postgres::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = users)]
pub struct RegisEntity {
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub amount: f64,
}

