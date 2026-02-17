use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};

#[derive(Clone)]
struct AppState {
    server_url: String,
    http: reqwest::Client,
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "healthy" }))
}

async fn ready(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let url = format!("{}/health", state.server_url);
    state
        .http
        .get(&url)
        .send()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .error_for_status()
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(json!({ "status": "ready" })))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let _ = dotenvy::dotenv();

    let server_url =
        std::env::var("DENTAL_SERVER_URL").expect("DENTAL_SERVER_URL must be set");

    tracing::info!("Proxying to dental-server at {server_url}");

    let state = AppState {
        server_url,
        http: reqwest::Client::new(),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .with_state(state);

    let addr = "0.0.0.0:8080";
    tracing::info!("dental-gateway listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
