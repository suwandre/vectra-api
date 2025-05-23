use ethers::types::Signature;
use ethers::utils::hash_message;
use std::str::FromStr;

/// Recovers the wallet address that signed the given message.
/// Returns `Some(address)` if valid, or `None` if recovery fails.
pub fn recover_wallet_address(message: &str, signature: &str) -> Option<String> {
    // Try to parse the signature string into a Signature object
    let signature = Signature::from_str(signature).ok()?;

    // Hash the message using the Ethereum-specific prefix
    let message_hash = hash_message(message);

    // Recover the public address that signed this hash
    let recovered_address = signature.recover(message_hash).ok()?;

    // Format the address (e.g., "0x123...") and return
    Some(format!("{:?}", recovered_address))
}