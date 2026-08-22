use axum::{routing::get, Router};

async fn health() -> &'static str {
    "orvpass-sync-ok"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8787")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
