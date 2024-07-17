use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Pool};
use dotenv::dotenv;

pub async fn connect() -> Pool<Postgres> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Failed to connect to Postgres")
}

pub async fn migrate() -> Result<(), sqlx::Error> {
    sqlx::migrate!("src/database/migrations")
        .run(&connect().await)
        .await
        .expect("Failed to migrate database");
    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub postgres_client: PgPool,
}