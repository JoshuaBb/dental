use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tonic::{transport::Server, Request, Response, Status};

pub mod dental {
    tonic::include_proto!("dental");
}

use dental::dental_service_server::{DentalService, DentalServiceServer};
use dental::{HealthRequest, HealthResponse, ReadyRequest, ReadyResponse};

struct DentalServiceImpl {
    pool: PgPool,
}

#[tonic::async_trait]
impl DentalService for DentalServiceImpl {
    async fn check_health(
        &self,
        request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let request_id = request
            .get_ref()
            .context
            .as_ref()
            .map(|c| c.request_id.as_str())
            .unwrap_or("unknown");
        tracing::info!(request_id, "check_health");
        Ok(Response::new(HealthResponse {
            status: "healthy".to_string(),
        }))
    }

    async fn check_ready(
        &self,
        request: Request<ReadyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        let request_id = request
            .get_ref()
            .context
            .as_ref()
            .map(|c| c.request_id.as_str())
            .unwrap_or("unknown");
        tracing::info!(request_id, "check_ready");
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| Status::unavailable(format!("database not ready: {e}")))?;
        Ok(Response::new(ReadyResponse {
            status: "ready".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let _ = dotenvy::dotenv();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database");

    let addr = "0.0.0.0:50051".parse()?;
    tracing::info!("dental-server listening on {addr}");

    Server::builder()
        .add_service(DentalServiceServer::new(DentalServiceImpl { pool }))
        .serve(addr)
        .await?;

    Ok(())
}
