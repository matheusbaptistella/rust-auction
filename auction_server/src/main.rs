use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

mod db;
mod user;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Error reading .env file");
    let db_pool = db::init().await;
    let app = router(db_pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn router(db_pool: PgPool) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/user", get(|| async { "Sign In" }))
        .route("/user", post(|| async { "Sign Up" }))
        .with_state(db_pool)
}
