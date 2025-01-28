use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn router() -> Router {
    Router::new().route("/", get(|| async { "Hello World!" }))
}
