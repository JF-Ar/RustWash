use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Pool};
use dotenv::dotenv;

async fn connect() -> Pool<Postgres> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Failed to connect to Postgres")
}

pub async fn migrate() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let pool = connect().await;
    sqlx::migrate!("src/database/migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");
    Ok(())
}