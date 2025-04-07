//! Error types for the Stripe API

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// A specialized Result type for Stripe operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when using the Stripe API
#[derive(Error, Debug)]
pub enum Error {
    /// Authentication error
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    /// API error returned by Stripe
    #[error("Stripe API error: {0}")]
    Api(#[from] ApiError),
    
    /// Rate limit error
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    
    /// Invalid request parameters
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),
    
    /// Timeout error
    #[error("Request timed out: {0}")]
    Timeout(String),
    
    /// Client configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Unexpected/unknown error
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::Timeout(err.to_string())
        } else if err.is_connect() {
            Error::Connection(err.to_string())
        } else if err.is_status() {
            match err.status() {
                Some(status) if status.as_u16() == 401 => {
                    Error::Authentication(err.to_string())
                }
                Some(status) if status.as_u16() == 429 => {
                    Error::RateLimit(err.to_string())
                }
                _ => Error::Api(ApiError {
                    error: ApiErrorDetail {
                        message: err.to_string(),
                        param: None,
                        code: None,
                        decline_code: None,
                        doc_url: None,
                        type_: ErrorType::Unknown,
                    },
                }),
            }
        } else {
            Error::Unexpected(err.to_string())
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}

/// Error type returned by the Stripe API
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ApiError {
    /// The error details
    pub error: ApiErrorDetail,
}

/// Detailed error information from the Stripe API
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ApiErrorDetail {
    /// Human-readable message
    pub message: String,
    
    /// Parameter that caused the error, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    
    /// Error code, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    
    /// Decline code for card errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_code: Option<String>,
    
    /// URL to documentation about this error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_url: Option<String>,
    
    /// Type of error
    #[serde(rename = "type")]
    pub type_: ErrorType,
}

/// Type of Stripe API error
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    /// API errors cover issues with the Stripe API or invalid Stripe requests
    ApiError,
    
    /// Card errors are the most common type of error you'll encounter
    CardError,
    
    /// Failure to authenticate
    AuthenticationError,
    
    /// Errors related to rate limits
    RateLimitError,
    
    /// Invalid request errors arise when your request has invalid parameters
    InvalidRequestError,
    
    /// Errors raised when Stripe's API servers are unavailable
    ApiConnectionError,
    
    /// Errors related to Stripe Connect
    ConnectError,
    
    /// Errors due to insufficient permissions
    PermissionError,
    
    /// Errors with Stripe-side issues
    StripeError,
    
    /// Unknown error type
    #[serde(other)]
    Unknown,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error.message)
    }
}

impl fmt::Display for ApiErrorDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ApiError {}