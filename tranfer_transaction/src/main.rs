use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tranfer_transaction::backend::{
    in_axum::http::start_serve, postgres::conn_postgres::establish_connection,
};
#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();
    let database = env::var("DATABASE_URL").expect("DATABASE_URL is invalid");

    let pool = establish_connection(&database).unwrap();
    start_serve(Arc::new(pool))
        .await
        .expect("serve don't start");
}
