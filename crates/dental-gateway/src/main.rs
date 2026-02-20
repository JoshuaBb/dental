use axum::Router;
use tonic::transport::Channel;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod routes;

pub mod dental {
    tonic::include_proto!("dental");
}

use dental::dental_service_client::DentalServiceClient;

pub const ORIGIN: &str = "dental";
const API_VERSION: &str = "v1";

#[derive(Clone)]
pub struct AppState {
    pub grpc: DentalServiceClient<Channel>,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::health,
        routes::ready,
        routes::status,
    ),
    components(
        schemas(
            routes::HealthApiResponse,
            routes::ReadyApiResponse,
            routes::StatusApiResponse,
        )
    ),
    tags(
        (name = "health", description = "Health and readiness check endpoints"),
    ),
    info(title = "Dental Gateway API", version = "1.0.0")
)]
pub struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let _ = dotenvy::dotenv();

    let server_url = std::env::var("DENTAL_SERVER_URL").expect("DENTAL_SERVER_URL must be set");

    tracing::info!("Connecting to dental-server gRPC at {server_url}");

    let channel = Channel::from_shared(server_url)?.connect_lazy();
    let grpc = DentalServiceClient::new(channel);

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest(&format!("/{API_VERSION}"), routes::router())
        .with_state(AppState { grpc });

    let addr = "0.0.0.0:8080";
    tracing::info!("dental-gateway listening on {addr}");
    tracing::info!("Swagger UI available at http://{addr}/swagger-ui");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
