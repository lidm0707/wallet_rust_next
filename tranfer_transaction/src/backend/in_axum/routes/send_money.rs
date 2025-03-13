use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{backend::postgres::{conn_postgres::PgPool, transaction_posetgres::TransPostgres}, domain::models::transaction_models::SendMoneyModel, service::transaction_service::SendMovieService};



pub async fn handler_send_money(
    State(pool): State<Arc<PgPool>>,
    Json(send_money_model): Json<SendMoneyModel>,
) -> impl IntoResponse {
    let trans_postgres = TransPostgres::new(pool);
    let service = SendMovieService::new(Arc::new(trans_postgres));
    match service.send_money_service(send_money_model).await {
        Ok(()) => (StatusCode::CREATED, format!("send successfully")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}