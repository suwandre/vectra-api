use axum::Router;

pub mod routes;

pub async fn create_router() -> Router {
    Router::new()
        .route("/", axum::routing::get(|| async { "Vectra DEX API v0.1" }))
        .route("/health", axum::routing::get(|| async { "API Health: OK" }))
        .nest("/auth", routes::auth::create_routes())
        .nest("/trading", routes::trading::create_routes())
}
