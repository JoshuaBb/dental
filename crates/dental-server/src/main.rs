use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use tracing_subscriber;

async fn health() -> Json<Value> {
    Json(json!({ "status": "healthy" }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/health", get(health));

    let addr = "0.0.0.0:3000";
    tracing::info!("dental-server listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
