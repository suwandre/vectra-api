//! Database models for the Vectra DEX application.
//! Contains all data structures that map to database tables.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// User account information.
/// Represents a user in the Vectra DEX platform with wallet-based authentication.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// Unique user identifier.
    pub id: Uuid,
    /// User's wallet address (used for authentication).
    pub wallet_address: String,
    /// User's display username (optional).
    pub username: Option<String>,
    /// User's current XP points for gamification.
    pub xp_points: i32,
    /// User's current seasonal level based on XP.
    pub level: i16,
    /// Total value of user's paper trading portfolio. Represented in cents.
    pub portfolio_value_cents: i64,
    /// Available cash balance for trading. Represented in cents.
    pub cash_balance_cents: i64,
    /// When the user account was created.
    pub created_at: DateTime<Utc>,
    /// When the user account was last updated.
    pub updated_at: DateTime<Utc>,
}

/// Paper trading transaction record.
/// Represents individual buy/sell transactions in the paper trading system.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Trade {
    /// Unique trade identifier.
    pub id: Uuid,
    /// User who executed the trade.
    pub user_id: Uuid,
    /// Trading symbol (e.g., "ETH", "BTC").
    pub symbol: String,
    /// Trade type: "buy" or "sell".
    pub trade_type: String,
    /// Number of tokens traded.
    pub quantity: f64,
    /// Price per token at execution.
    pub price: f64,
    /// Total trade value (quantity * price).
    pub total_value: f64,
    /// When the trade was executed.
    pub executed_at: DateTime<Utc>,
}

/// User's current portfolio positions.
/// Represents holdings of different cryptocurrencies in paper trading.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Position {
    /// Unique position identifier.
    pub id: Uuid,
    /// User who owns this position.
    pub user_id: Uuid,
    /// Trading symbol.
    pub symbol: String,
    /// Total quantity held.
    pub quantity: f64,
    /// Average purchase price.
    pub average_price: f64,
    /// Current market value of the position.
    pub current_value: f64,
    /// When the position was last updated.
    pub updated_at: DateTime<Utc>,
}

/// User session information.
/// Tracks active user sessions for authentication management.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSession {
    /// Unique session identifier.
    pub id: Uuid,
    /// User associated with this session.
    pub user_id: Uuid,
    /// JWT token for this session.
    pub token_hash: String,
    /// When the session expires.
    pub expires_at: DateTime<Utc>,
    /// When the session was created.
    pub created_at: DateTime<Utc>,
}
