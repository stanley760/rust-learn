use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

/// Main application error type
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Model error: {0}")]
    ModelError(String),

    #[error("Tokenization error: {0}")]
    TokenizationError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Training error: {0}")]
    TrainingError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Job not found: {0}")]
    JobNotFound(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Error response structure for API responses
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            AppError::InvalidInput(_) => (
                StatusCode::BAD_REQUEST,
                "BadRequest",
                self.to_string(),
            ),
            AppError::ModelNotFound(_) => (
                StatusCode::NOT_FOUND,
                "NotFound",
                self.to_string(),
            ),
            AppError::JobNotFound(_) => (
                StatusCode::NOT_FOUND,
                "NotFound",
                self.to_string(),
            ),
            AppError::ModelError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "ModelError",
                self.to_string(),
            ),
            AppError::TokenizationError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "TokenizationError",
                self.to_string(),
            ),
            AppError::TrainingError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "TrainingError",
                self.to_string(),
            ),
            AppError::IoError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "IoError",
                self.to_string(),
            ),
            AppError::ConfigError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "ConfigError",
                self.to_string(),
            ),
            AppError::InternalError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "InternalError",
                self.to_string(),
            ),
            AppError::SerializationError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "SerializationError",
                self.to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message,
            details: None,
        });

        (status, body).into_response()
    }
}

// Implement From trait for common error conversions
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err.to_string())
    }
}

impl From<toml::de::Error> for AppError {
    fn from(err: toml::de::Error) -> Self {
        AppError::ConfigError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input_error_status_code() {
        let error = AppError::InvalidInput("test error".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_model_not_found_error_status_code() {
        let error = AppError::ModelNotFound("model.bin".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_model_error_status_code() {
        let error = AppError::ModelError("failed to load".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_training_error_status_code() {
        let error = AppError::TrainingError("training failed".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_error_message_format() {
        let error = AppError::InvalidInput("empty string".to_string());
        let message = error.to_string();
        assert!(message.contains("Invalid input"));
        assert!(message.contains("empty string"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let app_error: AppError = io_error.into();
        assert!(matches!(app_error, AppError::IoError(_)));
    }

    #[test]
    fn test_serde_json_error_conversion() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json")
            .unwrap_err();
        let app_error: AppError = json_error.into();
        assert!(matches!(app_error, AppError::SerializationError(_)));
    }
}
