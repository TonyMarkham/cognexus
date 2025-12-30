use common::error::error_location::ErrorLocation;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Execution failed: {message} {location}")]
    ExecutionError {
        message: String,
        location: ErrorLocation,
    },

    #[error("Invalid input: {message} {location}")]
    InvalidInput {
        message: String,
        location: ErrorLocation,
    },

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
}
