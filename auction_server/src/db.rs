use sqlx::postgres::{PgPool, PgPoolOptions, PgQueryResult};
use crate::user::{CreateUser, User};

pub async fn init() -> PgPool {
    let db_url =
        std::env::var("DATABASE_URL").expect("Couldn't find environment variable DATABASE_URL");
    PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Couldn't create database connection pool")
}

pub async fn read_user(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT user_name, email
        FROM users
        WHERE users.email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn create_user(pool: &PgPool, user: CreateUser) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "INSERT INTO users (user_name, email, hashed_password, salt)
        VALUES ($1, $2, $3, $4);",
    )
    .bind(user.name)
    .bind(user.email)
    .bind(user.hashed_password)
    .bind(user.salt)
    .execute(pool)
    .await
}
