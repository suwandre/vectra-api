use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Represents a user account.
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
  /// A unique ID for the user.
    pub id: Uuid,
    /// The wallet address associated to the user.
    pub wallet_address: String,
    /// How much XP the user has accumulated on the current season.
    pub season_xp: i32,
    /// What level the user is on the current season. Attached to `season_xp`.
    pub season_level: i8,
    /// When this account was created.
    pub created_at: NaiveDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct LoginSession {
    pub wallet_address: String,
    pub nonce: String,
    pub created_at: NaiveDateTime,
}