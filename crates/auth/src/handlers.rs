use axum::{extract::State, Json};
use vectra_storage::repo::auth::{upsert_nonce, get_nonce};
use vectra_storage::repo::user::get_or_create_user;
use crate::{types::*, error::AppError, utils, crypto::recover_wallet_address};
use sqlx::PgPool;

/// Generates a login nonce for the given wallet address.
/// This message is later signed by the wallet to prove ownership.
pub async fn generate_nonce(
    Json(req): Json<NonceRequest>,     // Extract JSON body into NonceRequest
    State(pool): State<PgPool>,        // Inject the database connection pool
) -> Result<Json<NonceResponse>, AppError> {
    let nonce = utils::generate_nonce();      // Generate a secure random nonce
    let message = format!(             // Format the message to be signed by the user
        "Sign in to Vectra\nWallet: {}\nNonce: {}",
        req.wallet_address,
        nonce
    );

    upsert_nonce(&pool, &req.wallet_address, &nonce)    // Store nonce in DB
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(NonceResponse { message })) // Return the message to frontend
}

/// Verifies the signed message and logs the user in.
/// If the user doesn't exist, they're created on the fly.
pub async fn verify_signature(
    Json(req): Json<VerifyRequest>,    // Extract JSON body into VerifyRequest
    State(pool): State<PgPool>,        // Inject the database connection pool
) -> Result<Json<AuthResponse>, AppError> {
    // Fetch expected nonce from DB
    let Some(expected_nonce) = get_nonce(&pool, &req.wallet_address)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
    else {
        return Err(AppError::Unauthorized("Nonce not found".into()));
    };

    // Rebuild the expected message
    let expected_msg = format!(
        "Sign in to Vectra\nWallet: {}\nNonce: {}",
        req.wallet_address,
        expected_nonce
    );

    // Make sure the message exactly matches what the user signed
    if req.message != expected_msg {
        return Err(AppError::Unauthorized("Message mismatch".into()));
    }

    // Recover signer address from the signature
    let signer = recover_wallet_address(&req.message, &req.signature)
        .ok_or_else(|| AppError::Unauthorized("Invalid signature".into()))?;

    // Ensure recovered signer matches claimed wallet address
    if signer.to_lowercase() != req.wallet_address.to_lowercase() {
        return Err(AppError::Unauthorized("Signature doesn't match address".into()));
    }

    // Fetch or create user
    let user = get_or_create_user(&pool, &req.wallet_address)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // Issue a JWT for the session
    let token = issue_jwt(&user)?;

    // Respond with the token
    Ok(Json(AuthResponse { token }))
}