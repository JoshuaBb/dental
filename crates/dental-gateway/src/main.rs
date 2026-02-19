use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};
use tonic::transport::Channel;

pub mod dental {
    tonic::include_proto!("dental");
}

use dental::dental_service_client::DentalServiceClient;
use dental::{HealthRequest, ReadyRequest};

#[derive(Clone)]
struct AppState {
    grpc: DentalServiceClient<Channel>,
}

async fn health(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let mut client = state.grpc.clone();
    let resp = client
        .check_health(HealthRequest {})
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(json!({ "status": resp.into_inner().status })))
}

async fn ready(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let mut client = state.grpc.clone();
    let resp = client
        .check_ready(ReadyRequest {})
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(json!({ "status": resp.into_inner().status })))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let _ = dotenvy::dotenv();

    let server_url = std::env::var("DENTAL_SERVER_URL").expect("DENTAL_SERVER_URL must be set");

    tracing::info!("Connecting to dental-server gRPC at {server_url}");

    let channel = Channel::from_shared(server_url)?.connect_lazy();
    let grpc = DentalServiceClient::new(channel);

    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .with_state(AppState { grpc });

    let addr = "0.0.0.0:8080";
    tracing::info!("dental-gateway listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
