//! Data structures for API requests and responses.
//! These types define the shape of data flowing through the Vectra DEX API.

use serde::{Deserialize, Serialize};
use validator::Validate;
use std::sync::LazyLock;
use regex::Regex;

// Validation regex patterns

/// Regex pattern for validating Ethereum wallet addresses.
/// Matches addresses that start with 0x followed by 40 hexadecimal characters.
static WALLET_ADDRESS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap()
});

// Wallet authentication related types

/// Request to initiate wallet connection process.
/// First step in wallet authentication where client provides their wallet address.
#[derive(Deserialize, Validate)]
pub struct WalletConnectRequest {
    /// User's wallet address (must be valid Ethereum address format).
    #[validate(length(equal = 42, message = "Wallet address must be 42 characters"))]
    #[validate(regex(path = "WALLET_ADDRESS_REGEX", message = "Invalid wallet address format"))]
    pub wallet_address: String,
}

/// Response containing nonce for wallet signature.
/// Provides the message that needs to be signed by the user's wallet.
#[derive(Serialize)]
pub struct NonceResponse {
    /// Random nonce for wallet signature.
    pub nonce: String,
    /// Message to be signed by the wallet.
    pub message: String,
}

/// Request to complete wallet authentication.
/// Second step where client provides the signed message for verification.
#[derive(Deserialize, Validate)]
pub struct WalletLoginRequest {
    /// User's wallet address.
    #[validate(length(equal = 42, message = "Wallet address must be 42 characters"))]
    #[validate(regex(path = "WALLET_ADDRESS_REGEX", message = "Invalid wallet address format"))]
    pub wallet_address: String,
    /// Signature created by signing the nonce message.
    #[validate(length(min = 130, max = 132, message = "Invalid signature length"))]
    pub signature: String,
    /// The nonce that was signed.
    #[validate(length(min = 1, message = "Nonce cannot be empty"))]
    pub nonce: String,
}

/// Successful authentication response.
/// Contains user session information and authentication token.
#[derive(Serialize)]
pub struct AuthResponse {
    /// JWT token for authenticated requests.
    pub token: String,
    /// Unique user identifier.
    pub user_id: String,
    /// User's wallet address.
    pub wallet_address: String,
    /// User's current XP level (for gamification).
    pub level: u32,
    /// Whether this is a new user registration.
    pub is_new_user: bool,
}

// Trading related types

/// User's complete portfolio information.
/// Contains all positions, balances, and portfolio metrics for paper trading.
#[derive(Serialize)]
pub struct Portfolio {
    /// Total portfolio value in USD.
    pub total_value: f64,
    /// Available cash for trading.
    pub cash_balance: f64,
    /// List of current positions.
    pub positions: Vec<Position>,
}

/// Individual trading position.
/// Represents a single cryptocurrency holding in the portfolio.
#[derive(Serialize)]
pub struct Position {
    /// Trading symbol (e.g., "ETH", "BTC").
    pub symbol: String,
    /// Number of tokens held.
    pub quantity: f64,
    /// Average purchase price.
    pub avg_price: f64,
    /// Current market value.
    pub current_value: f64,
}

/// Individual trade record.
/// Represents a single executed trade in the paper trading system.
#[derive(Serialize)]
pub struct Trade {
    /// Unique trade identifier.
    pub id: String,
    /// Trading symbol.
    pub symbol: String,
    /// Trade type: "buy" or "sell".
    pub trade_type: String,
    /// Number of tokens traded.
    pub quantity: f64,
    /// Price per token.
    pub price: f64,
    /// When the trade was executed.
    pub timestamp: String,
}

// Generic API response wrapper

/// Standardized API response wrapper.
/// Provides consistent response format across all endpoints.
#[derive(Serialize)]
pub struct ApiResponse<T> {
    /// Whether the request was successful.
    pub success: bool,
    /// Response data (if successful).
    pub data: Option<T>,
    /// Success or informational message.
    pub message: Option<String>,
}
