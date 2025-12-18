use common::error::error_location::ErrorLocation;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Model Error: {message} {location}")]
    ModelError {
        message: String,
        location: ErrorLocation,
    },

    #[error("Camera Error: {message} {location}")]
    CameraError {
        message: String,
        location: ErrorLocation,
    },
}
