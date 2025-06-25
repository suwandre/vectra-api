//! Error types and handling for the Vectra DEX API.
//! Provides standardized error responses and conversion from various error types.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

/// Main error type for all API operations.
/// Represents different categories of errors that can occur in the application.
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Authentication failed: {message}")]
    Authentication { message: String },
    
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },
    
    #[error("Internal server error: {message}")]
    Internal { message: String },
    
    #[error("Bad request: {message}")]
    BadRequest { message: String },
}

/// Standardized error response structure.
/// Provides consistent error format across all API endpoints.
#[derive(Serialize)]
pub struct ErrorResponse {
    /// Always false for error responses.
    pub success: bool,
    /// Human-readable error message.
    pub message: String,
    /// Error code for programmatic handling.
    pub error_code: String,
}

impl IntoResponse for ApiError {
    /// Converts ApiError into HTTP response with appropriate status code.
    /// Maps different error types to their corresponding HTTP status codes.
    fn into_response(self) -> Response {
        let (status, error_code, message) = match self {
            ApiError::Validation { message } => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", message),
            ApiError::Authentication { message } => (StatusCode::UNAUTHORIZED, "AUTH_ERROR", message),
            ApiError::NotFound { resource } => (StatusCode::NOT_FOUND, "NOT_FOUND", format!("{} not found", resource)),
            ApiError::Internal { message } => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", message),
            ApiError::BadRequest { message } => (StatusCode::BAD_REQUEST, "BAD_REQUEST", message),
        };

        let error_response = ErrorResponse {
            success: false,
            message,
            error_code: error_code.to_string(),
        };

        (status, Json(error_response)).into_response()
    }
}

/// Convenience type alias for API results.
/// Simplifies function signatures throughout the application.
pub type ApiResult<T> = Result<T, ApiError>;