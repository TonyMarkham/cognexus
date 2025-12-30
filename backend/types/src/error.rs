use common::error::error_location::ErrorLocation;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Serialization failed: {message} {location}")]
    SerializationError {
        message: String,
        location: ErrorLocation,
    },

    #[error("Deserialization failed: {message} {location}")]
    DeserializationError {
        message: String,
        location: ErrorLocation,
    },

    #[error("Type mismatch: expected {expected}, got {got} {location}")]
    TypeMismatch {
        expected: String,
        got: String,
        location: ErrorLocation,
    },
}
