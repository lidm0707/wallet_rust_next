use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    backend::postgres::{conn_postgres::PgPool, regis_postgres::RegisPostgres},
    domain::models::resgis_model::RegisModel,
    service::regis_service::RegisService,
};

pub async fn handler_resgistor(
    State(pool): State<Arc<PgPool>>,
    Json(user_registor_model): Json<RegisModel>,
) -> impl IntoResponse {
    let resgis_postgres = RegisPostgres::new(pool);
    let service = RegisService::new(Arc::new(resgis_postgres));
    match service.registor_service(user_registor_model).await {
        Ok(()) => (StatusCode::CREATED, format!("Register successfully")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
