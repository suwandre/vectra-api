//! Database configuration and connection management.
//! Handles PostgreSQL connection pooling and environment-based configuration.

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

/// Database configuration structure.
/// Contains all settings needed to establish and manage database connections.
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// PostgreSQL connection URL.
    pub database_url: String,
    /// Maximum number of connections in the pool.
    pub max_connections: u32,
    /// Connection timeout in seconds.
    pub connection_timeout: u64,
}

impl DatabaseConfig {
    /// Creates database configuration from Elastic Beanstalk environment variables.
    /// Uses RDS_* variables provided by EB when database is integrated.
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // Check if running in Elastic Beanstalk with integrated RDS
        if let (Ok(host), Ok(db_name), Ok(username), Ok(password)) = (
            std::env::var("RDS_HOSTNAME"),
            std::env::var("RDS_DB_NAME"),
            std::env::var("RDS_USERNAME"),
            std::env::var("RDS_PASSWORD"),
        ) {
            let port = std::env::var("RDS_PORT").unwrap_or_else(|_| "5432".to_string());
            let database_url = format!(
                "postgres://{}:{}@{}:{}/{}",
                username, password, host, port, db_name
            );
            
            Ok(Self {
                database_url,
                max_connections: 10,
                connection_timeout: 30,
            })
        } else {
            // Fallback to DATABASE_URL for local development
            let database_url = std::env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL or RDS environment variables required")?;
            
            Ok(Self {
                database_url,
                max_connections: 10,
                connection_timeout: 30,
            })
        }
    }
}


/// Creates and returns a configured PostgreSQL connection pool.
/// Establishes connection pool with proper timeout and connection limits.
pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(Duration::from_secs(config.connection_timeout))
        .connect(&config.database_url)
        .await
}

/// Tests database connectivity.
/// Performs a simple query to verify the database connection is working.
pub async fn test_connection(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;
    
    Ok(())
}