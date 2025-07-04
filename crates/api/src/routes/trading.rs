use axum::{routing::get, Router};

use crate::state::SharedState;

pub fn create_routes() -> Router<SharedState> {
    Router::new()
        .route("/portfolio", get(get_portfolio))
        .route("/trades", get(get_trades))
}

async fn get_portfolio() -> &'static str {
    "Portfolio endpoint - Paper trading portfolio"
}

async fn get_trades() -> &'static str {
    "Trades endpoint - Paper trading history"
}