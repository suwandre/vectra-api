use axum::{routing::get, Router};

/// Creates a router with all routes.
pub fn router() -> axum::Router {
    Router::new()
        .route("/", get(health_check))
}

/// Simple health check to ensure that the backend is alive.
async fn health_check() -> &'static str {
    "Vectra backend is alive!"
}