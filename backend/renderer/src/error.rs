use common::error::error_location::ErrorLocation;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RendererError {
    #[error("WGPU Error: {message} {location}")]
    WgpuError {
        message: String,
        location: ErrorLocation,
    },
}
