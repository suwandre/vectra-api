//! User-related database queries.
//! Handles user creation, authentication, and profile management.

use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::User;

/// Creates a new user account with wallet address.
/// Initializes user with default XP, level, and starting cash balance.
pub async fn create_user(
    pool: &PgPool,
    wallet_address: &str,
    username: Option<&str>,
) -> Result<User, sqlx::Error> {
    let user_id = Uuid::new_v4();
    let now = Utc::now();
    
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, wallet_address, username, xp_points, level, portfolio_value_cents, cash_balance_cents, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
        user_id,
        wallet_address,
        username,
        0i32,       // PostgreSQL INTEGER (will be cast to u32)
        1i16,       // PostgreSQL SMALLINT (will be cast to u8)
        1000000i64, // $10,000 in cents
        1000000i64, // $10,000 in cents
        now,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Finds a user by their wallet address.
pub async fn find_user_by_wallet(
    pool: &PgPool,
    wallet_address: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE wallet_address = $1",
        wallet_address
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Finds a user by their unique ID.
pub async fn find_user_by_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}


/// Updates user's XP points and recalculates level.
/// Used for gamification features when users complete actions.
pub async fn update_user_xp(
    pool: &PgPool,
    user_id: Uuid,
    xp_points: u32,  // ✅ Changed to u32 for consistency
) -> Result<User, sqlx::Error> {
    let level = game::calculate_level_from_xp(xp_points);
    let now = Utc::now();
    
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users 
        SET xp_points = $2, level = $3, updated_at = $4
        WHERE id = $1
        RETURNING *
        "#,
        user_id,
        xp_points as i32,  // Cast u32 to i32 for PostgreSQL
        level as i16,      // Cast u8 to i16 for PostgreSQL
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}