//! Vectra DEX main application entry point.
//! Configures and starts the Axum server with proper AWS Elastic Beanstalk integration.

use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

    // Create the main application router
    let app = create_app().await?;

    // Get port from environment variable (Elastic Beanstalk requirement)
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .unwrap_or(5000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    
    info!("🚀 Vectra DEX server starting on http://{}", addr);
    info!("📡 API available at http://{}/api/v1", addr);
    info!("🏥 Health check at http://{}/health", addr);

    // Start the server
    axum::serve(listener, app).await?;

    Ok(())
}

/// Creates the main application router with health check for Elastic Beanstalk.
/// Configures all routes and middleware for production deployment.
async fn create_app() -> Result<Router, Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Vectra DEX - More Than a DEX. It's an Arena." }))
        // Health check endpoint required by Elastic Beanstalk load balancer
        .route("/health", axum::routing::get(health_check))
        .nest("/api/v1", api::create_router().await);

    Ok(app)
}

/// Health check endpoint for Elastic Beanstalk load balancer.
/// Returns 200 OK when the application is healthy and ready to serve requests.
async fn health_check() -> &'static str {
    "OK"
}
