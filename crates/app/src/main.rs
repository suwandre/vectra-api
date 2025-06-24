use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create the main application router
    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Vectra DEX - More Than a DEX. It's an Arena." }))
        .nest("/api/v1", api::create_router().await);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;
    
    info!("🚀 Vectra DEX server starting on http://{}", addr);
    info!("📡 API available at http://{}/api/v1", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
