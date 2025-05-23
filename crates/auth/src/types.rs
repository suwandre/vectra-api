use serde::{Deserialize, Serialize};

/// Represents a nonce request instance.
#[derive(Deserialize)]
pub struct NonceRequest {
  /// The wallet address of the user requesting a nonce.
    pub wallet_address: String,
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
