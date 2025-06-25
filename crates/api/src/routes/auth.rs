//! Wallet authentication routes for MetaMask and other Web3 wallets.
//! Handles wallet connection, nonce generation, and signature verification.

use axum::{routing::post, Router, Json};
use crate::types::{WalletConnectRequest, NonceResponse, WalletLoginRequest, AuthResponse, ApiResponse};
use crate::auth_utils::{generate_nonce, create_sign_message, verify_wallet_signature};
use crate::errors::{ApiError, ApiResult};
use crate::middleware::validate_request;

/// Creates authentication route group for wallet-based auth.
/// Provides endpoints for wallet connection and signature verification.
pub fn create_routes() -> Router {
    Router::new()
        .route("/wallet/connect", post(wallet_connect))
        .route("/wallet/login", post(wallet_login))
}

/// Initiates wallet connection by providing a nonce to sign.
/// Client calls this first to get a message to sign with their wallet.
async fn wallet_connect(
    Json(payload): Json<WalletConnectRequest>,
) -> ApiResult<Json<ApiResponse<NonceResponse>>> {
    // Validate request data
    validate_request(&payload)?;

    // Generate nonce and create message to sign
    let nonce = generate_nonce();
    let message = create_sign_message(&nonce);

    let nonce_response = NonceResponse {
        nonce: nonce.clone(),
        message,
    };

    let response = ApiResponse {
        success: true,
        data: Some(nonce_response),
        message: Some("Please sign the message with your wallet.".to_string()),
    };

    Ok(Json(response))
}

/// Completes wallet authentication by verifying the signed message.
/// Client calls this after signing the nonce message with their wallet.
async fn wallet_login(
    Json(payload): Json<WalletLoginRequest>,
) -> ApiResult<Json<ApiResponse<AuthResponse>>> {
    // Validate request data
    validate_request(&payload)?;

    // Recreate the message that should have been signed
    let expected_message = create_sign_message(&payload.nonce);

    // Verify the signature
    match verify_wallet_signature(&expected_message, &payload.signature, &payload.wallet_address) {
        Ok(true) => {
            // Signature is valid - create user session
            // TODO: Check if user exists in database, create if new
            
            let auth_response = AuthResponse {
                token: format!("jwt_token_for_{}", &payload.wallet_address[..10]), // Mock JWT
                user_id: format!("user_{}", &payload.wallet_address[2..8]), // Mock user ID
                wallet_address: payload.wallet_address.clone(),
                level: 1, // New users start at level 1
                is_new_user: true, // TODO: Check actual database
            };

            let response = ApiResponse {
                success: true,
                data: Some(auth_response),
                message: Some("Welcome to Vectra DEX! Your wallet has been authenticated.".to_string()),
            };

            Ok(Json(response))
        }
        Ok(false) => {
            // This will now return 401 Unauthorized instead of 200 OK
            Err(ApiError::Authentication {
                message: "Invalid signature. Please try signing the message again.".to_string(),
            })
        }
        Err(e) => {
            // This will return 500 Internal Server Error
            Err(ApiError::Internal {
                message: format!("Error verifying signature: {}", e),
            })
        }
    }
}
