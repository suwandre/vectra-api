use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::error::AppError;
use uuid::Uuid;

/// Claims stored inside the JWT token.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject: the user ID
    pub sub: String,
    /// Expiration timestamp (UNIX time)
    pub exp: usize,
}

/// Issues a JWT for the given user ID.
pub fn issue_jwt(user_id: &Uuid) -> Result<String, AppError> {
    // Set token expiration (e.g., 24 hours from now)
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    // Create the payload claims
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    // Load secret key from environment variable
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::Internal("Missing JWT_SECRET env var".into()))?;

    // Encode the JWT using HS256
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("JWT encoding failed: {}", e)))
}
