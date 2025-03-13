use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{
    backend::{
        jwt_authentication::passport_model::Passport,
        postgres::{authen_postgres::AuthenPostgres, conn_postgres::PgPool},
    },
    domain::models::authen_model::AuthenModel,
    service::authen_service::AuthenService,
};

pub async fn hanlder_login(
    State(pool): State<Arc<PgPool>>,
    Json(user_login_model): Json<AuthenModel>,
) -> impl IntoResponse {
    let authen_postgres = AuthenPostgres::new(pool);
    let service = AuthenService::new(Arc::new(authen_postgres));

    match service.login_service(user_login_model).await {
        Ok(passport) => (
            StatusCode::CREATED,
            Json(json!({
                "message": "Login success",
                "data": passport
            })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn hanlder_refresh_token(
    State(pool): State<Arc<PgPool>>,
    Json(passort): Json<Passport>,
) -> impl IntoResponse {
    let authen_postgres = AuthenPostgres::new(pool);
    let service = AuthenService::new(Arc::new(authen_postgres));

    match service.refresh_token_service(passort).await {
        Ok(new_passort) => (
            StatusCode::CREATED,
            Json(json!({
                "message": "Login success",
                "data": new_passort
            })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
