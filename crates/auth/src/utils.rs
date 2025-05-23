use rand::{distr::Alphanumeric, Rng};

/// Generates a random nonce for use when logging in via wallet.
pub fn generate_nonce() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
} 