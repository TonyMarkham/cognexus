use thiserror::Error;
use common::error::error_location::ErrorLocation;

#[derive(Error, Debug)]
pub enum RendererError {
    #[error("WGPU Error: {message} {location:?}")]
    WgpuError {
        message: String,
        location: Option<ErrorLocation>,
    }
}