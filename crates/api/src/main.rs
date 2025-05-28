use dotenv::dotenv;
use routes::router;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up structured logging, with filter from RUST_LOG
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Load environment variables from .env
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL must be set in .env");

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

    // Build Axum router (with attached HTTP tracing layer)
    let app = router(db)
    .layer(cors)
    .layer(TraceLayer::new_for_http());

    // Bind and serve
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server is running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
