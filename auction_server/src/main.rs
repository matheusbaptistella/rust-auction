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
        .route("/users/me", get(user::sign_in))
        .route("/users", post(user::sign_up))
        .with_state(db_pool)
}
