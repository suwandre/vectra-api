//! Middleware for request processing and validation.
//! Handles CORS, request logging, and validation across all API endpoints.

use axum::{extract::Request, http::Method, middleware::Next, response::Response};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use validator::Validate;

use crate::errors::ApiError;

/// Creates CORS layer for cross-origin requests.
/// Allows frontend applications to communicate with the API from different domains.
pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any) // TODO: Restrict to specific origins in production
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
}

/// Logs incoming requests for debugging and monitoring.
/// Records request method, path, and processing time.
pub async fn request_logger(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    info!("📥 {} {}", method, path);

    let response = next.run(request).await;

    info!("📤 {} {} - Status: {}", method, path, response.status());

    response
}

/// Validates request data using the validator crate.
/// Ensures all incoming data meets the defined validation rules.
pub fn validate_request<T: Validate>(data: &T) -> Result<(), ApiError> {
    match data.validate() {
        Ok(_) => Ok(()),
        Err(validation_errors) => {
            let error_messages: Vec<String> = validation_errors
                .field_errors()
                .iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(move |error| {
                        format!(
                            "{}: {}",
                            field,
                            error.message.as_ref().unwrap_or(&"Invalid value".into())
                        )
                    })
                })
                .collect();

            Err(crate::errors::ApiError::Validation {
                message: error_messages.join(", "),
            })
        }
    }
}
