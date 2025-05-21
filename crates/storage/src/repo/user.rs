use sqlx::PgPool;
use anyhow::Result;
use crate::schema::User;

/// Gets or creates a user.
pub async fn get_or_create_user(pool: &PgPool, address: &str) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE wallet_address = $1",
        address
    )
    .fetch_optional(pool)
    .await?;

    if let Some(u) = user {
        return Ok(u);
    }

    let new_user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (wallet_address)
        VALUES ($1)
        RETURNING id, wallet_address, created_at
        "#,
        address
    )
    .fetch_one(pool)
    .await?;

    Ok(new_user)
}
