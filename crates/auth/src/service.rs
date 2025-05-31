use crate::error::AuthError;
use crate::jwt::issue_jwt;
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;
use vectra_storage::repo::auth::insert_refresh_token;

/// Issues a short-lived access JWT plus a long-lived refresh token (and persists it).
#[instrument(skip(pool))]
pub async fn issue_tokens(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<(String /* access */, String /* refresh */), AuthError> {
    tracing::info!(%user_id, "Issuing access + refresh tokens");

    // 1) Access token from the JWT layer
    let access = issue_jwt(&user_id)?;

    // 2) Create & hash a refresh token
    let raw_refresh = Uuid::new_v4().to_string();
    let mut hasher = Sha256::new();
    hasher.update(&raw_refresh);
    let refresh_hash = format!("{:x}", hasher.finalize());

    // 3) Persist with long expiry (e.g. 30 days)
    let expires_at = Utc::now() + Duration::days(30);
    insert_refresh_token(pool, user_id, &refresh_hash, expires_at)
        .await
        .map_err(AuthError::Database)?;

    Ok((access, raw_refresh))
}
