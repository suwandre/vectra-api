use axum::{routing::get, Router};
use vectra_auth::routes::auth_routes;

/// Creates a router with all routes.
pub fn router() -> axum::Router {
    Router::new()
        .route("/", get(health_check))
        .nest("/auth", auth_routes())
}

/// Simple health check to ensure that the backend is alive.
async fn health_check() -> &'static str {
    "Vectra backend is alive!"
}