use axum::{routing::post, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register() -> &'static str {
    "Register endpoint - Coming soon"
}

async fn login() -> &'static str {
    "Login endpoint - Coming soon"
}
