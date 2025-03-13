use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::backend::postgres::schema::trans;

#[derive(Debug, Clone, Deserialize, Serialize, Selectable, Queryable, Insertable)]
#[diesel(table_name = trans)]
pub struct SendMoneyEntity {
    pub sender: i32,
    pub receiver: i32,
    pub amount: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
