//! Database integration for Vectra DEX.
//! Provides PostgreSQL connectivity, models, and query operations.

pub mod config;
pub mod models;

// Re-export commonly used items
pub use config::{DatabaseConfig, create_pool, test_connection};
pub use models::*;

/// Database query modules.
/// Contains organized query functions for different data domains.
pub mod queries {
    pub mod users;
}
