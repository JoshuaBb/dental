use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::{json, Value};
use tokio::join;

use crate::dental::{HealthRequest, ReadyRequest};
use crate::{AppState, ORIGIN};

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

async fn health(State(state): State<AppState>) -> Result<PassThrough, StatusCode> {
    let resp = state
        .grpc
        .clone()
        .check_health(HealthRequest {})
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(PassThrough(json!({ "status": resp.into_inner().status })))
}

async fn ready(State(state): State<AppState>) -> Result<PassThrough, StatusCode> {
    let resp = state
        .grpc
        .clone()
        .check_ready(ReadyRequest {})
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(PassThrough(json!({ "status": resp.into_inner().status })))
}

// ---------------------------------------------------------------------------
// Custom JSON routes — fan out to multiple gRPC calls, build bespoke JSON
// ---------------------------------------------------------------------------

async fn status(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let (mut health_client, mut ready_client) = (state.grpc.clone(), state.grpc.clone());
    let (health_res, ready_res) = join!(
        health_client.check_health(HealthRequest {}),
        ready_client.check_ready(ReadyRequest {}),
    );

    let health_status = health_res
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .into_inner()
        .status;
    let ready_status = ready_res
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .into_inner()
        .status;

    Ok(Json(json!({
        "health": health_status,
        "ready": ready_status,
    })))
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
