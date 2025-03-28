use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> Result<PgPool> {
    dotenv().ok();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;
    Ok(pool)
}
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/backend/postgres/migrations");

pub fn run_migrations(conn: &mut PgPool) -> Result<(), anyhow::Error> {
    conn.get()?.run_pending_migrations(MIGRATIONS).unwrap();
    Ok(())
}
