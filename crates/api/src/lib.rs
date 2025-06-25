//! Vectra DEX API library.
//! Contains all HTTP route definitions and handlers for the gamified DEX.

use axum::{middleware as axum_middleware, Router};
use tower_http::trace::TraceLayer;

pub mod routes;
pub mod types;
pub mod auth_utils;
pub mod errors;
pub mod middleware;

/// Creates the main API router with all endpoint groups and middleware.
/// Configures CORS, logging, and error handling for all routes.
pub async fn create_router() -> Router {
    Router::new()
        .route("/", axum::routing::get(|| async { "Vectra DEX API v0.1" }))
        .route("/health", axum::routing::get(|| async { "API Health: OK" }))
        // Group authentication endpoints under /auth
        .nest("/auth", routes::auth::create_routes())
        // Group trading endpoints under /trading  
        .nest("/trading", routes::trading::create_routes())
        // Add middleware layers
        .layer(axum_middleware::from_fn(middleware::request_logger))
        .layer(middleware::create_cors_layer())
        .layer(TraceLayer::new_for_http())
}
