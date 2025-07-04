//! Wallet authentication routes for MetaMask and other Web3 wallets.
//! Handles wallet connection, nonce generation, and signature verification.

use axum::{routing::post, Router, Json};
use db::queries::users;
use crate::state::SharedState;
use crate::types::{WalletConnectRequest, NonceResponse, WalletLoginRequest, AuthResponse, ApiResponse};
use crate::auth_utils::{generate_nonce, create_sign_message, verify_wallet_signature};
use crate::errors::{ApiError, ApiResult};
use crate::middleware::validate_request;

/// Creates authentication route group for wallet-based auth.
/// Provides endpoints for wallet connection and signature verification.
pub fn create_routes() -> Router<SharedState> {
    Router::new()
        .route("/wallet/connect", post(wallet_connect))
        .route("/wallet/login", post(wallet_login))
}

/// Initiates wallet connection by providing a nonce to sign.
/// Client calls this first to get a message to sign with their wallet.
async fn wallet_connect(
    State(state): State<SharedState>,
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
    State(state): State<SharedState>,
    Json(payload): Json<WalletLoginRequest>,
) -> ApiResult<Json<ApiResponse<AuthResponse>>> {
    validate_request(&payload)?;

    let expected_message = create_sign_message(&payload.nonce);

    match verify_wallet_signature(&expected_message, &payload.signature, &payload.wallet_address) {
        Ok(true) => {
            // Replace mock data with real database operations
            match users::find_user_by_wallet(&state.db_pool, &payload.wallet_address).await {
                Ok(Some(existing_user)) => {
                    // Existing user login
                    let auth_response = AuthResponse {
                        token: format!("jwt_token_for_{}", &payload.wallet_address[..10]), // TODO: Real JWT
                        user_id: existing_user.id.to_string(),
                        wallet_address: existing_user.wallet_address,
                        level: existing_user.level as u32,
                        is_new_user: false,
                    };

                    let response = ApiResponse {
                        success: true,
                        data: Some(auth_response),
                        message: Some("Welcome back to Vectra DEX!".to_string()),
                    };

                    Ok(Json(response))
                },
                Ok(None) => {
                    // Create new user
                    match users::create_user(&state.db_pool, &payload.wallet_address, None).await {
                        Ok(new_user) => {
                            let auth_response = AuthResponse {
                                token: format!("jwt_token_for_{}", &payload.wallet_address[..10]), // TODO: Real JWT
                                user_id: new_user.id.to_string(),
                                wallet_address: new_user.wallet_address,
                                level: new_user.level as u32,
                                is_new_user: true,
                            };

                            let response = ApiResponse {
                                success: true,
                                data: Some(auth_response),
                                message: Some("Welcome to Vectra DEX! Your account has been created.".to_string()),
                            };

                            Ok(Json(response))
                        },
                        Err(_) => Err(ApiError::Internal {
                            message: "Failed to create user account".to_string(),
                        })
                    }
                },
                Err(_) => Err(ApiError::Internal {
                    message: "Database connection failed".to_string(),
                })
            }
        }
        Ok(false) => {
            Err(ApiError::Authentication {
                message: "Invalid signature. Please try signing the message again.".to_string(),
            })
        }
        Err(e) => {
            Err(ApiError::Internal {
                message: format!("Error verifying signature: {}", e),
            })
        }
    }
}
