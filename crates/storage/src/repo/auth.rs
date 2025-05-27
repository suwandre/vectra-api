use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

/// Inserts or updates a nonce for a wallet login session into the database.
pub async fn upsert_nonce(pool: &PgPool, address: &str, nonce: &str) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO login_sessions (wallet_address, nonce)
        VALUES ($1, $2)
        ON CONFLICT (wallet_address) DO UPDATE SET nonce = $2, created_at = now()
        "#,
        address,
        nonce
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Gets a stored nonce **and its creation timestamp** for a given wallet address.
pub async fn get_nonce(
    pool: &PgPool,
    address: &str,
) -> Result<Option<(String, DateTime<Utc>)>> {
    let rec = sqlx::query!(
        r#"
        SELECT nonce, created_at
          FROM login_sessions
         WHERE wallet_address = $1
        "#,
        address
    )
    .fetch_optional(pool)
    .await?;
    Ok(rec.map(|r| (r.nonce, r.created_at)))
}

/// Deletes a login session (nonce) after use or expiry.
pub async fn delete_login_session(pool: &PgPool, address: &str) -> Result<()> {
    sqlx::query!(
        "DELETE FROM login_sessions WHERE wallet_address = $1",
        address
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Persists a new refresh token hash for a user, with an expiry timestamp.
pub async fn insert_refresh_token(
    pool: &PgPool,
    user_id: Uuid,
    token_hash: &str,
    expires_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3)
        "#,
        user_id,
        token_hash,
        expires_at,
    )
    .execute(pool)
    .await?;
    Ok(())
}