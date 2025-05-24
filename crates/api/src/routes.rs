use axum::{routing::get, Router};
use sqlx::PgPool;
use vectra_auth::routes::auth_routes;

/// Creates a router with all routes.
pub fn router(db: PgPool) -> axum::Router {
    Router::new()
        .route("/", get(health_check))
        .nest("/auth", auth_routes())
        .with_state(db)
}

/// Simple health check to ensure that the backend is alive.
async fn health_check() -> &'static str {
    "Vectra backend is alive!"
}