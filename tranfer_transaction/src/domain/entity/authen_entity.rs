use crate::backend::postgres::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = users)]
pub struct AuthenEntity {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub amount: f64,
}
