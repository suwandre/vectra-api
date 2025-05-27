use axum::{Router, routing::post};
use sqlx::PgPool;
use crate::handlers::{generate_nonce, refresh_tokens, verify_signature};

/// Returns a router with all auth-related endpoints.
/// These will be mounted under `/auth` in the main router.
pub fn auth_routes() -> Router<PgPool> {
    Router::new()
        .route("/nonce", post(generate_nonce))
        .route("/verify", post(verify_signature))
        .route("/refresh", post(refresh_tokens))
}