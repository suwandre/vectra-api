//! Data structures for API requests and responses
//! These types define the shape of data flowing through the Vectra DEX API

use serde::{Deserialize, Serialize};


// Wallet authentication related types
#[derive(Deserialize)]
pub struct WalletConnectRequest {
    /// User's wallet address (public key)
    pub wallet_address: String,
}

#[derive(Serialize)]
pub struct NonceResponse {
    /// Random nonce for wallet signature
    pub nonce: String,
    /// Message to be signed by the wallet
    pub message: String,
}

#[derive(Deserialize)]
pub struct WalletLoginRequest {
    /// User's wallet address
    pub wallet_address: String,
    /// Signature created by signing the nonce message
    pub signature: String,
    /// The nonce that was signed
    pub nonce: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    /// JWT token for authenticated requests
    pub token: String,
    /// Unique user identifier
    pub user_id: String,
    /// User's wallet address
    pub wallet_address: String,
    /// User's current XP level (for gamification)
    pub level: u32,
    /// Whether this is a new user registration
    pub is_new_user: bool,
}

// Trading related types
#[derive(Serialize)]
pub struct Portfolio {
    /// Total portfolio value in USD
    pub total_value: f64,
    /// Available cash for trading
    pub cash_balance: f64,
    /// List of current positions
    pub positions: Vec<Position>,
}

#[derive(Serialize)]
pub struct Position {
    /// Trading symbol (e.g., "ETH", "BTC")
    pub symbol: String,
    /// Number of tokens held
    pub quantity: f64,
    /// Average purchase price
    pub avg_price: f64,
    /// Current market value
    pub current_value: f64,
}

#[derive(Serialize)]
pub struct Trade {
    /// Unique trade identifier
    pub id: String,
    /// Trading symbol
    pub symbol: String,
    /// Trade type: "buy" or "sell"
    pub trade_type: String,
    /// Number of tokens traded
    pub quantity: f64,
    /// Price per token
    pub price: f64,
    /// When the trade was executed
    pub timestamp: String,
}

// Generic API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    /// Whether the request was successful
    pub success: bool,
    /// Response data (if successful)
    pub data: Option<T>,
    /// Error message (if failed)
    pub message: Option<String>,
}