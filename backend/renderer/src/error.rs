use common::error::error_location::ErrorLocation;
use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum RendererError {
    #[error("WGPU Error: {message} {location}")]
    WgpuError {
        message: String,
        location: ErrorLocation,
    },

    #[error("Command Error: {message} {location}")]
    CommandError {
        message: String,
        location: ErrorLocation,
    },
}

impl From<RendererError> for JsValue {
    fn from(err: RendererError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}
