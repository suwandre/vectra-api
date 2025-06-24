use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize basic logging
    tracing_subscriber::fmt::init();

    // Create a simple router
    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Vectra DEX - More Than a DEX. It's an Arena." }))
        .route("/health", axum::routing::get(|| async { "OK" }));

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    
    info!("🚀 Vectra DEX server starting on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}