//! Vectra DEX main application entry point.
//! Configures and starts the Axum server with proper AWS Elastic Beanstalk integration.

use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use sqlx;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

    // Initialize database connection
    let db_pool = match initialize_database().await {
        Ok(pool) => {
            info!("✅ Database connection established");
            Some(pool)
        },
        Err(e) => {
            tracing::warn!("⚠️ Database connection failed: {}", e);
            tracing::warn!("🔄 Starting without database - limited functionality");
            None
        }
    };

    // Create the main application router
    let app = create_app(db_pool).await?;

    // Get port from environment (EB uses 5000 by default)
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .unwrap_or(5000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    
    info!("🚀 Vectra DEX server starting on http://{}", addr);
    info!("📡 API available at http://{}/api/v1", addr);
    info!("🏥 Health check at http://{}/health", addr);
    info!("🏥 Database health at http://{}/api/v1/health/db", addr);

    // Start the server
    axum::serve(listener, app).await?;

    Ok(())
}

/// Initializes database connection pool.
/// Returns connection pool or error if database is unavailable.
async fn initialize_database() -> Result<sqlx::PgPool, Box<dyn std::error::Error>> {
    let config = DatabaseConfig::from_env()?;
    let pool = create_pool(&config).await?;
    test_connection(&pool).await?;
    Ok(pool)
}

/// Health check endpoint for Elastic Beanstalk load balancer.
/// Returns 200 OK when the application is healthy and ready to serve requests.
async fn health_check() -> &'static str {
    "OK"
}


/// Creates the main application router with health check for Elastic Beanstalk.
/// Configures all routes and middleware for production deployment.
async fn create_app(db_pool: sqlx::PgPool) -> Result<Router, Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Vectra DEX - More Than a DEX. It's an Arena." }))
        // Health check endpoint required by Elastic Beanstalk load balancer
        .route("/health", axum::routing::get(health_check))
        .nest("/api/v1", api::create_router(db_pool).await);

    Ok(app)
}

/// Health check endpoint for Elastic Beanstalk load balancer.
/// Returns 200 OK when the application is healthy and ready to serve requests.
async fn health_check() -> &'static str {
    "OK"
}
