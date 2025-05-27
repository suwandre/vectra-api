use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a SIWE nonce request.
#[derive(Deserialize)]
pub struct NonceRequest {
    /// The wallet address of the user requesting a nonce.
    pub wallet_address: String,
    /// The domain requesting sign-in, e.g. "vectra.app"
    pub domain: String,
    /// The URI of the app, e.g. "https://vectra.app"
    pub uri: String,
    /// The chain ID the user is on, e.g. 1 for Ethereum mainnet
    pub chain_id: u64,
}

/// Represents a nonce response instance.
#[derive(Serialize)]
pub struct NonceResponse {
    /// The message and nonce generated for the user.
    pub message: String,
}

/// Represents a signature verification request instance.
#[derive(Deserialize)]
pub struct VerifyRequest {
    /// The wallet address of the user used upon signature verification.
    pub wallet_address: String,
    /// The message (including the nonce) used upon signature verification. Uses raw EIP-4361 format.
    pub message: String,
    /// The underlying EIP-191 or EIP-712 signature of the user.
    pub signature: String,
}

/// Server → client after login (initial verify) or refresh
#[derive(Serialize)]
pub struct AuthResponse {
    /// Short‐lived access JWT (e.g. 15m)
    pub access_token: String,
    /// Long‐lived refresh token (e.g. 30d)
    pub refresh_token: String,
}

/// Client → server when asking for new access + refresh tokens
#[derive(Deserialize)]
pub struct RefreshRequest {
    /// The user’s UUID
    pub user_id: Uuid,
    /// The raw refresh token string
    pub refresh_token: String,
}

/// Server → client with the new access + refresh tokens
#[derive(Serialize)]
pub struct RefreshResponse {
    /// New short‐lived access JWT
    pub access_token: String,
    /// New (or same) refresh token
    pub refresh_token: String,
}