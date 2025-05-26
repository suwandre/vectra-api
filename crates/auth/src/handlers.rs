use crate::jwt::issue_jwt;
use crate::{crypto::recover_wallet_address, error::AppError, types::*, utils};
use axum::http::uri::Authority;
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use chrono::{Duration, TimeZone, Utc};
use ethers::types::Address;
use iri_string::spec::UriSpec;
use iri_string::types::RiString;
use sqlx::PgPool;
use siwe::{Message, TimeStamp, Version};
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
    let siwe_msg = Message {
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
