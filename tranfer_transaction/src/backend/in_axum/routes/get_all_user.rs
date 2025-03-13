use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{backend::postgres::{conn_postgres::PgPool, get_user_postgres::UserPostgres}, service::get_user_service::GetUserService};

pub async fn hanlder_get_all_user(
    State(pool): State<Arc<PgPool>>,
) -> impl IntoResponse {
    let get_user_postgres = UserPostgres::new(pool);
    let service = GetUserService::new(Arc::new(get_user_postgres));
    match service.get_all_user().await{
        Ok(data) => (
            StatusCode::CREATED,
            Json(json!({
                "message": "get success",
                "data": data
            })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}