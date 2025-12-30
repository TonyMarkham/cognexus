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

    #[error("Port Error: {message} (port: '{port_name}', type: {data_type_id}) {location}")]
    PortError {
        message: String,
        port_name: String,
        data_type_id: uuid::Uuid,
        location: ErrorLocation,
    },
}
