use axum::Router;
use tonic::transport::Channel;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let _ = dotenvy::dotenv();

    let server_url = std::env::var("DENTAL_SERVER_URL").expect("DENTAL_SERVER_URL must be set");

    tracing::info!("Connecting to dental-server gRPC at {server_url}");

    let channel = Channel::from_shared(server_url)?.connect_lazy();
    let grpc = DentalServiceClient::new(channel);

    // TODO: BUG here
    let app = Router::new()
        .nest(&format!("/{API_VERSION}"), routes::router())
        .with_state(AppState { grpc });

    let addr = "0.0.0:8080";
    tracing::info!("dental-gateway listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
