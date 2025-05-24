use crate::jwt::issue_jwt;
use crate::{crypto::recover_wallet_address, error::AppError, types::*, utils};
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use chrono::{Duration, TimeZone, Utc};
use sqlx::PgPool;
use vectra_storage::repo::auth::{delete_login_session, get_nonce, upsert_nonce};
use vectra_storage::repo::user::get_or_create_user;

/// Generates a login nonce for the given wallet address.
/// This message is later signed by the wallet to prove ownership.
#[debug_handler]
pub async fn generate_nonce(
    State(pool): State<PgPool>,    // Inject the database connection pool
    Json(req): Json<NonceRequest>, // Extract JSON body into NonceRequest
) -> Result<Json<NonceResponse>, AppError> {
    let nonce = utils::generate_nonce(); // Generate a secure random nonce
    let message = format!(
        // Format the message to be signed by the user
        "Sign in to Vectra\nWallet: {}\nNonce: {}",
        req.wallet_address, nonce
    );

    upsert_nonce(&pool, &req.wallet_address, &nonce) // Store nonce in DB
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(NonceResponse { message })) // Return the message to frontend
}

/// Verifies the signed message and logs the user in.
/// If the user doesn't exist, they're created on the fly.
#[debug_handler]
pub async fn verify_signature(
    State(pool): State<PgPool>,
    Json(req): Json<VerifyRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // 1) Fetch stored nonce + timestamp option
    let (expected_nonce, created_at) = get_nonce(&pool, &req.wallet_address)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::Unauthorized("Nonce not found".into()))?;

    // 2) Convert to UTC DateTime and enforce a 10-minute TTL
    let created_at_utc = Utc.from_utc_datetime(&created_at);
    if Utc::now().signed_duration_since(created_at_utc) > Duration::minutes(10) {
        delete_login_session(&pool, &req.wallet_address).await.ok();
        return Err(AppError::Unauthorized("Nonce expired".into()));
    }

    // 3) Rebuild and compare the expected message
    let expected_msg = format!(
        "Sign in to Vectra\nWallet: {}\nNonce: {}",
        req.wallet_address, expected_nonce
    );
    if req.message != expected_msg {
        delete_login_session(&pool, &req.wallet_address).await.ok();
        return Err(AppError::Unauthorized("Message mismatch".into()));
    }

    // 4) Recover signer and verify address
    let signer = recover_wallet_address(&req.message, &req.signature)
        .ok_or_else(|| AppError::Unauthorized("Invalid signature".into()))?;
    if signer.to_lowercase() != req.wallet_address.to_lowercase() {
        delete_login_session(&pool, &req.wallet_address).await.ok();
        return Err(AppError::Unauthorized(
            "Signature doesn't match address".into(),
        ));
    }

    // 5) Create or fetch the user record
    let user = get_or_create_user(&pool, &req.wallet_address)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // 6) Issue a JWT
    let token = issue_jwt(&user.id)?;

    // 7) Invalidate the nonce on successful login
    delete_login_session(&pool, &req.wallet_address)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // 8) Return the token
    Ok(Json(AuthResponse { token }))
}
