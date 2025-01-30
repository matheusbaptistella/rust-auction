use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn init() -> PgPool {
    let db_url =
        std::env::var("DATABASE_URL").expect("Couldn't find environment variable DATABASE_URL");
    PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Couldn't create database connection pool")
}

async fn create_user() -> Result<(), &'static str> {}
