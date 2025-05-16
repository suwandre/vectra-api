use axum::extract::Extension;
use routes::router;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use tower_http::cors::{CorsLayer, Any};
use dotenv::dotenv;

mod routes;

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
    let app = router()
        .layer(Extension(db))
        .layer(cors);

    // Bind and serve
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
