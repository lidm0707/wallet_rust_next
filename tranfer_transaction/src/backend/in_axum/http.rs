use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{
    http::Method,
    middleware,
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

use crate::{
    backend::{
        in_axum::{
            middleware::token::extract_bearer_token,
            routes::{
                get_all_user::hanlder_get_all_user, login::{hanlder_login, hanlder_refresh_token}, registor::handler_resgistor, send_money::handler_send_money
            },
        },
        postgres::conn_postgres::PgPool,
    },
    service::say_hi_user::hi_user,
};

pub async fn start_serve(db_pool: Arc<PgPool>) -> Result<()> {
    let authen_route = Router::new()
        .route("/login", post(hanlder_login))
        .route("/regis", post(handler_resgistor))
        .route("/claim_token", post(hanlder_refresh_token));
    let transac = Router::new()
        .route("/send_money", post(handler_send_money))
        .layer(middleware::from_fn(extract_bearer_token));

    let app = Router::new()
        .route("/", get(hi_user))
        .route("/get_users", get(hanlder_get_all_user))
        .nest("/authen", authen_route)
        .nest("/trans", transac)
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any)
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ]),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(db_pool);
    let port = 8080;

    let socket = SocketAddr::new([0, 0, 0, 0].into(), port);
    let listener = TcpListener::bind(socket).await?;
    info!("start serve {}", port);
    axum::serve(listener, app).await?;

    Ok(())
}
