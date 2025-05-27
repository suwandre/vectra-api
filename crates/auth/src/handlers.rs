use crate::service::issue_tokens;
use crate::{error::AppError, types::*, utils};
use axum::http::uri::Authority;
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use chrono::{Duration, Utc};
use ethers::types::Address;
use ethers::utils::hex;
use iri_string::spec::UriSpec;
use iri_string::types::RiString;
use siwe::{Message as SiweMessage, TimeStamp, VerificationOpts, Version};
use sqlx::PgPool;
use time::OffsetDateTime;
use vectra_storage::repo::auth::{delete_login_session, get_nonce, upsert_nonce};
use vectra_storage::repo::user::get_or_create_user;

/// Generates a login nonce for the given wallet address.
/// This message is later signed by the wallet to prove ownership.
#[debug_handler]
pub async fn generate_nonce(
    State(pool): State<PgPool>,    // Inject the database connection pool
    Json(req): Json<NonceRequest>, // Extract JSON body into NonceRequest
) -> Result<Json<NonceResponse>, AppError> {
    // Generate the random nonce and instantiate all the required fields for `siwe_msg`
    let nonce = utils::generate_nonce();

    let domain: Authority = req
        .domain
        .parse()
        .map_err(|e| AppError::BadRequest(format!("Invalid domain: {}", e)))?;

    let eth_addr: Address = req
        .wallet_address
        .parse()
        .map_err(|e| AppError::BadRequest(format!("Invalid wallet address: {}", e)))?;
    let address: [u8; 20] = eth_addr.0;

    let uri: RiString<UriSpec> = req
        .uri
        .parse()
        .map_err(|e| AppError::BadRequest(format!("Invalid URI: {}", e)))?;

    let now: TimeStamp = OffsetDateTime::now_utc().into();

    // Build the SIWE message
    let siwe_msg = SiweMessage {
        domain,
        address,
        statement: Some("Sign in to Vectra".into()),
        uri,
        version: Version::V1,
        chain_id: req.chain_id,
        nonce: nonce.clone(),
        issued_at: now,
        expiration_time: None,
        not_before: None,
        request_id: None,
        resources: Vec::new(),
    };

    // Serialize the message into the human-readable string
    let message = siwe_msg.to_string();

    // Store the nonce in DB (created_at set automatically)
    upsert_nonce(&pool, &req.wallet_address, &nonce)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // Return the SIWE message
    Ok(Json(NonceResponse { message }))
}

/// Verifies the signed message and logs the user in.
/// If the user doesn't exist, they're created on the fly.
#[debug_handler]
pub async fn verify_signature(
    State(pool): State<PgPool>,
    Json(req): Json<VerifyRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // 1) Fetch stored nonce + timestamp
    let (expected_nonce, created_at) = get_nonce(&pool, &req.wallet_address)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::Unauthorized("Nonce not found".into()))?;

    // 2) Enforce 10-minute TTL
    if Utc::now().signed_duration_since(created_at) > Duration::minutes(10) {
        delete_login_session(&pool, &req.wallet_address).await.ok();
        return Err(AppError::Unauthorized("Nonce expired".into()));
    }

    // 3) Parse the SIWE message
    let siwe_msg = req
        .message
        .parse::<SiweMessage>()
        .map_err(|e| AppError::Unauthorized(format!("Malformed SIWE message: {}", e)))?;

    // 4) Ensure the nonce matches
    if siwe_msg.nonce != expected_nonce {
        delete_login_session(&pool, &req.wallet_address).await.ok();
        return Err(AppError::Unauthorized("Nonce mismatch".into()));
    }

    // 5) Decode the hex signature into bytes
    let sig_bytes = hex::decode(req.signature.trim_start_matches("0x"))
        .map_err(|e| AppError::BadRequest(format!("Invalid signature format: {}", e)))?;

    // 6) Verify the signature
    let opts = VerificationOpts::default();
    siwe_msg
        .verify(&sig_bytes, &opts)
        .await
        .map_err(|e| AppError::Unauthorized(format!("Signature verification failed: {}", e)))?;

    // 7) Parse the claimed address string into [u8; 20]
    let expected_addr: Address = req
        .wallet_address
        .parse()
        .map_err(|e| AppError::BadRequest(format!("Invalid wallet address: {}", e)))?;
    let expected_addr_bytes = expected_addr.0;

    // 8) Compare the two 20-byte addresses
    if siwe_msg.address != expected_addr_bytes {
        delete_login_session(&pool, &req.wallet_address).await.ok();
        return Err(AppError::Unauthorized("Address mismatch".into()));
    }

    // 9) Fetch-or-create the user
    let user = get_or_create_user(&pool, &req.wallet_address)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // 10) Issue both access and refresh tokens
    let (access_token, refresh_token) = issue_tokens(&pool, user.id).await?;

    // 11) Invalidate the nonce
    delete_login_session(&pool, &req.wallet_address)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // 12) Return both tokens
    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
    }))
}

/// Refreshes the user’s authentication tokens.
///
/// Given a valid `RefreshRequest` (with `user_id` + `refresh_token`),  
/// this endpoint verifies the refresh token, then issues a new short-lived  
/// access JWT and a new (or rotated) refresh token.  
///
/// Returns `RefreshResponse { access_token, refresh_token }`.
#[debug_handler]
pub async fn refresh_tokens(
    State(pool): State<PgPool>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, AppError> {
    let (access, refresh) = issue_tokens(&pool, req.user_id).await?;
    Ok(Json(RefreshResponse {
        access_token: access,
        refresh_token: refresh,
    }))
}
