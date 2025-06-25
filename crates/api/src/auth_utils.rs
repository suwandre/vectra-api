//! Wallet authentication utilities
//! Handles wallet signature verification and nonce generation

use ethers::{types::{Address, Signature}};
use uuid::Uuid;

/// Generates a random nonce for wallet signature.
/// Returns a unique string that the wallet will sign.
pub fn generate_nonce() -> String {
    Uuid::new_v4().to_string()
}

/// Creates the message that the wallet should sign.
/// Includes nonce and application context for security.
pub fn create_sign_message(nonce: &str) -> String {
    format!(
        "Welcome to Vectra DEX!\n\nSign this message to authenticate your wallet.\n\nNonce: {}\n\nThis request will not trigger a blockchain transaction or cost any gas fees.",
        nonce
    )
}

/// Verifies that the signature was created by the claimed wallet address.
/// Uses ethers-rs to recover the address from the signature.
pub fn verify_wallet_signature(
    message: &str,
    signature: &str,
    expected_address: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Hash the message (Ethereum-specific message prefix)
    let msg_hash = ethers::utils::hash_message(message);

    // Parse the signature
    let signature = signature.parse::<Signature>()?;

    // Recover the address from the signature
    let recovered_address = signature.recover(msg_hash)?;

    // Parse the expected address
    let expected = expected_address.parse::<Address>()?;

    Ok(recovered_address == expected)
}

/// Validates that a wallet address has the correct format
/// Checks if it's a valid Ethereum address format
pub fn is_valid_wallet_address(address: &str) -> bool {
    // Basic validation: starts with 0x and is 42 characters long
    address.starts_with("0x") && address.len() == 42 && address[2..].chars().all(|c| c.is_ascii_hexdigit())
}
