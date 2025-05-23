use sqlx::PgPool;
use anyhow::Result;

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

/// Gets a stored nonce for a given wallet address from the database.
pub async fn get_nonce(pool: &PgPool, address: &str) -> Result<Option<String>> {
    let rec = sqlx::query!(
        "SELECT nonce FROM login_sessions WHERE wallet_address = $1",
        address
    )
    .fetch_optional(pool)
    .await?;

    Ok(rec.map(|r| r.nonce))
}