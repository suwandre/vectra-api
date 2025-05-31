use axum::{
    http::StatusCode,
    response::{IntoResponse},
    Json,
};
use serde::Serialize;
use anyhow;
use thiserror::Error;
use sqlx::Error as SqlxError;
use siwe::VerificationError;
use jsonwebtoken::errors::Error as JwtError;
use anyhow::Error as AnyhowError;

#[derive(Debug, Error)]
pub enum AuthError {
    /// 400 Bad Request: malformed JSON, missing fields, parse errors, etc.
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// 401 Unauthorized: incorrect signature, nonce mismatch, expired token, etc.
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// 500 Internal Server Error: any database error (e.g., failure to SELECT or INSERT).
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    /// 401 Unauthorized: failure parsing/verifying the SIWE message or signature.
    #[error("SIWE verification error: {0}")]
    Siwe(#[from] VerificationError),

    /// 401 Unauthorized: any JWT encoding/decoding/validation error.
    #[error("JWT error: {0}")]
    Jwt(#[from] JwtError),

    /// 500 Internal Server Error: catch-all for other unexpected errors, wrapped in anyhow::Error.
    #[error("Internal server error: {0}")]
    Internal(#[from] AnyhowError),
}

/// Represents the JSON body for error responses.
#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

/// Implements `IntoResponse` for `AuthError` to convert it into an specific HTTP response.
impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        // Decide status code + message based on the variant
        let (status, message) = match &self {
            AuthError::BadRequest(msg)      => (StatusCode::BAD_REQUEST, msg.clone()),
            AuthError::Unauthorized(msg)    => (StatusCode::UNAUTHORIZED, msg.clone()),
            AuthError::Database(err)        => {
                // For DB errors, avoid leaking raw SQL details; send a generic message
                tracing::error!("Auth database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
            AuthError::Siwe(err)            => {
                // SIWE failures often indicate a bad signature or malformed message
                (
                    StatusCode::UNAUTHORIZED,
                    err.to_string(), 
                )
            }
            AuthError::Jwt(err)             => {
                // JWT failures indicate something wrong with token creation/verification
                (
                    StatusCode::UNAUTHORIZED,
                    err.to_string(),
                )
            }
            AuthError::Internal(err)        => {
                // Catch-all for unexpected errors; log the details, but don’t leak them to clients
                tracing::error!("Internal auth error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        // Build a JSON body: { "error": "<message>" }
        let body = Json(ErrorBody { error: message });
        (status, body).into_response()
    }
}