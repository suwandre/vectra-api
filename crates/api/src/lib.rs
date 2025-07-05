//! Vectra DEX API library.
//! Contains all HTTP route definitions and handlers for the gamified DEX.

use std::sync::Arc;

use axum::{extract::State, middleware as axum_middleware, Router};
use tower_http::trace::TraceLayer;
use sqlx;

pub mod routes;
pub mod types;
pub mod auth_utils;
pub mod errors;
pub mod middleware;
pub mod state; 

use state::{AppState, SharedState};

/// Creates the main API router with all endpoint groups and middleware.
/// Configures CORS, logging, error handling and shared DB connection pool for all routes.
pub async fn create_router(db_pool: sqlx::PgPool) -> Router {
    // Create shared application state
    let app_state = Arc::new(AppState::new(db_pool));

    Router::new()
        .route("/", axum::routing::get(|| async { "Vectra DEX API v0.1" }))
        .route("/health", axum::routing::get(|| async { "API Health: OK" }))
        .route("/health/db", axum::routing::get(health_check_db))
        // Group authentication endpoints under /auth
        .nest("/auth", routes::auth::create_routes())
        // Group trading endpoints under /trading  
        .nest("/trading", routes::trading::create_routes())
        // Add middleware layers
        .layer(axum_middleware::from_fn(middleware::request_logger))
        .layer(middleware::create_cors_layer())
        .layer(TraceLayer::new_for_http())
        // Add shared application state
        .with_state(app_state)
}

/// Database health check endpoint.
/// Verifies database connectivity and returns status.
async fn health_check_db(
    State(state): State<SharedState>
) -> Result<String, String> {
    match sqlx::query("SELECT 1").execute(&state.db_pool).await {
        Ok(_) => Ok("Database connection healthy".to_string()),
        Err(e) => Err(format!("Database connection failed: {}", e)),
    }
}