use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a user account.
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    /// A unique ID for the user.
    pub id: Uuid,
    /// The wallet address associated to the user.
    pub wallet_address: String,
    /// When this account was created.
    pub created_at: DateTime<Utc>,
}

/// Represents a session instance when a user logs in.
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct LoginSession {
    /// The wallet address tied to the user.
    pub wallet_address: String,
    /// A randomly generated nonce accompanying the login instance.
    pub nonce: String,
    /// When this login session was created.
    pub created_at: DateTime<Utc>,
}
