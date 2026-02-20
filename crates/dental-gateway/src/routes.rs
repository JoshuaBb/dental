use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use serde_json::{json, Value};
use tokio::join;
use utoipa::ToSchema;

use crate::dental::{HealthRequest, ReadyRequest};
use crate::{AppState, ORIGIN};

// ---------------------------------------------------------------------------
// API response schemas
// ---------------------------------------------------------------------------

#[derive(Serialize, ToSchema)]
pub struct HealthApiResponse {
    /// Service health status
    pub status: String,
    /// Origin service name
    pub origin: String,
}

#[derive(Serialize, ToSchema)]
pub struct ReadyApiResponse {
    /// Service readiness status
    pub status: String,
    /// Origin service name
    pub origin: String,
}

#[derive(Serialize, ToSchema)]
pub struct StatusApiResponse {
    /// Service health status
    pub health: String,
    /// Service readiness status
    pub ready: String,
}

// ---------------------------------------------------------------------------
// Pass-through helpers
// ---------------------------------------------------------------------------

/// Wraps a gRPC result as JSON and injects `"origin"` automatically.
pub struct PassThrough(pub Value);

// TODO: BUG here
impl IntoResponse for PassThrough {
    fn into_response(self) -> axum::response::Response {
        let mut value = self.0;
        if let Some(obj) = value.as_object_mut() {
            obj.insert("origin".to_string(), json!(ORIGIN));
        }
        Json(value).into_response()
    }
}

// ---------------------------------------------------------------------------
// Pass-through routes — single gRPC call, "origin" added automatically
// ---------------------------------------------------------------------------

#[utoipa::path(
    get,
    path = "/v1/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthApiResponse),
        (status = 503, description = "Service unavailable"),
    )
)]
pub async fn health(State(state): State<AppState>) -> Result<PassThrough, StatusCode> {
    let resp = state
        .grpc
        .clone()
        .check_health(HealthRequest { context: None })
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(PassThrough(json!({ "status": resp.into_inner().status })))
}

#[utoipa::path(
    get,
    path = "/v1/ready",
    tag = "health",
    responses(
        (status = 200, description = "Service is ready", body = ReadyApiResponse),
        (status = 503, description = "Service unavailable"),
    )
)]
pub async fn ready(State(state): State<AppState>) -> Result<PassThrough, StatusCode> {
    let resp = state
        .grpc
        .clone()
        .check_ready(ReadyRequest { context: None })
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(PassThrough(json!({ "status": resp.into_inner().status })))
}

// ---------------------------------------------------------------------------
// Custom JSON routes — fan out to multiple gRPC calls, build bespoke JSON
// ---------------------------------------------------------------------------

#[utoipa::path(
    get,
    path = "/v1/status",
    tag = "health",
    responses(
        (status = 200, description = "Combined health and readiness status", body = StatusApiResponse),
        (status = 503, description = "Service unavailable"),
    )
)]
pub async fn status(State(state): State<AppState>) -> Result<Json<StatusApiResponse>, StatusCode> {
    let (mut health_client, mut ready_client) = (state.grpc.clone(), state.grpc.clone());
    let (health_res, ready_res) = join!(
        health_client.check_health(HealthRequest { context: None }),
        ready_client.check_ready(ReadyRequest { context: None }),
    );

    let health_status = health_res
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .into_inner()
        .status;
    let ready_status = ready_res
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .into_inner()
        .status;

    Ok(Json(StatusApiResponse {
        health: health_status,
        ready: ready_status,
    }))
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

/// All API route handlers. Mount this under the version prefix in `main`.
pub fn router() -> Router<AppState> {
    Router::new()
        // Pass-through routes
        .route("/health", get(health))
        .route("/ready", get(ready))
        // Custom JSON routes
        .route("/status", get(status))
}
