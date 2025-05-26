use serde::{Deserialize, Serialize};

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
    /// The message (including the nonce) used upon signature verification.
    pub message: String,
    /// The underlying signature of the user.
    pub signature: String,
}

/// Represents an authentication response instance.
#[derive(Serialize)]
pub struct AuthResponse {
    /// The authentication token generated for the user.
    pub token: String,
}
