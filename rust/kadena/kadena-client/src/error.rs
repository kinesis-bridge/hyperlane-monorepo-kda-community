use thiserror::Error;

use crate::models::{
    EventParam,
    EventParamType,
};

#[derive(Error, Debug)]
pub enum KadenaClientError {
    #[error("Mismatch in event parameter type: expected {expected:?}, got {actual:?}")]
    EventParamsTypeMismatchError {
        expected: EventParamType,
        actual: EventParam,
    },

    #[error("Mismatch in the number of event parameters: expected {expected}, got {actual}")]
    EventParamsCountMismatchError { expected: usize, actual: usize },

    #[error("HTTP request error: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("Base64 decode error: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("Custom type conversion error: {0}")]
    TypeConversionError(String),

    #[error("Failed to parse integer: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Failed to parse float: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error("Received an error response with status code {0} and content: {1}")]
    HttpResponseError(reqwest::StatusCode, String),

    #[error("Failed to deserialize JSON: {0}")]
    DeserializationError(#[from] serde_json::Error),

    #[error("Poll timeout error: {0:?}")]
    PollTimeoutError(Vec<String>),

    #[error("Transaction result error: {0}")]
    TransactionResultError(String),

    #[error("Ed25519 Error: {0}")]
    Ed25519Error(#[from] ed25519_dalek::ed25519::Error),

    #[error("Other error: {0}")]
    OtherError(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("Overflow error")]
    OverflowError,
}
