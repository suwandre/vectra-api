use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::error::AuthError;
use uuid::Uuid;

/// Claims stored inside the JWT token.
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Subject: the user ID
    sub: Uuid,
    /// Issued At (as UTC timestamp)
    iat: usize,
    /// Expiration time (UTC timestamp)
    exp: usize,
    /// JWT ID: a unique identifier for this token
    jti: String,
}

/// Issues a JWT for the given user ID.
pub fn issue_jwt(user_id: &Uuid) -> Result<String, AuthError> {
    // 1) Load secret key
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AuthError::Internal(anyhow::anyhow!("Missing JWT_SECRET env var")))?;
    let key = EncodingKey::from_secret(secret.as_bytes());

    // 2) Build timestamps
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(15)).timestamp() as usize;

    // 3) Create claims
    let claims = Claims {
        sub: *user_id,
        iat,
        exp,
        jti: Uuid::new_v4().to_string(),
    };

    // 4) Encode with HS256
    encode(
        &Header {
            alg: Algorithm::HS256,
            ..Default::default()
        },
        &claims,
        &key,
    )
    .map_err(AuthError::Jwt)
}
