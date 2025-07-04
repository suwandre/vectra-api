//! Application state management for API handlers.
//! Provides shared database connection pool and configuration.

use sqlx::PgPool;
use std::sync::Arc;

/// Shared application state containing database connection pool.
/// Used by all API handlers to access the database.
#[derive(Clone)]
pub struct AppState {
    /// PostgreSQL connection pool for database operations.
    pub db_pool: PgPool,
}

impl AppState {
    /// Creates a new application state with database pool.
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

/// Type alias for shared application state.
/// Makes it easier to use in handler function signatures.
pub type SharedState = Arc<AppState>;
