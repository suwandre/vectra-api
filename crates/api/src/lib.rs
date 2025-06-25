//! Vectra DEX API library
//! Contains all HTTP route definitions and handlers for the gamified DEX

use axum::Router;

pub mod routes;
pub mod types;
pub mod auth_utils;

/// Creates the main API router with all endpoint groups
pub async fn create_router() -> Router {
    Router::new()
        .route("/", axum::routing::get(|| async { "Vectra DEX API v0.1" }))
        .route("/health", axum::routing::get(|| async { "API Health: OK" }))
        // Group authentication endpoints under /auth
        .nest("/auth", routes::auth::create_routes())
        // Group trading endpoints under /trading  
        .nest("/trading", routes::trading::create_routes())
}