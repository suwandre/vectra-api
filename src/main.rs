use axum::{
    routing::get,
    Router,
    extract::Extension,
};
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use tower_http::cors::{CorsLayer, Any};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // Connect to PostgreSQL
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // CORS layer for dev (permissive)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build Axum router
    let app = Router::new()
        .route("/", get(health_check))
        .layer(Extension(db))
        .layer(cors);

    // Bind and serve
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// Example handler
async fn health_check() -> &'static str {
    "Vectra backend is alive!"
}
