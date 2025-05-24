use axum::{
    http::StatusCode,
    response::{IntoResponse},
    Json,
};
use serde_json::json;
use anyhow;

/// Represents all possible application-level errors that can occur for authentication-related functionality.
/// Converts errors into HTTP responses using Axum's `IntoResponse` trait.
#[derive(Debug)]
pub enum AppError {
    /// Returned when authentication fails (e.g. bad signature or nonce mismatch).
    Unauthorized(String),
    /// Returned when the request is malformed or invalid.
    BadRequest(String),
    /// Returned when something unexpected fails internally (e.g. DB failure).
    Internal(String),
}

/// Converts `AppError` into an HTTP response using Axum's `IntoResponse` trait.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Unauthorized(msg) => {
                // 401 Unauthorized with a JSON error message
                let body = Json(json!({ "error": msg }));
                (StatusCode::UNAUTHORIZED, body).into_response()
            }
            AppError::BadRequest(msg) => {
                // 400 Bad Request with a JSON error message
                let body = Json(json!({ "error": msg }));
                (StatusCode::BAD_REQUEST, body).into_response()
            }
            AppError::Internal(msg) => {
                // 500 Internal Server Error with a generic message
                let body = Json(json!({
                    "error": "Internal server error",
                    "details": msg 
                }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}

/// Allows using `?` with `anyhow::Error` or other fallible calls
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}