use common::error::error_location::ErrorLocation;

use std::panic::Location;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[allow(dead_code)]
    #[error("Common Error: {message} {location}")]
    InspectError {
        message: String,
        location: ErrorLocation,
    },

    #[error("WASM Error: {message} {location}")]
    Wasm {
        message: String,
        location: ErrorLocation,
    },
}

impl CliError {
    #[track_caller]
    pub fn from_wasm(error: wasmtime::Error) -> Self {
        CliError::Wasm {
            message: error.to_string(),
            location: ErrorLocation::from(Location::caller()),
        }
    }
}

impl From<wasmtime::Error> for CliError {
    #[track_caller]
    fn from(error: wasmtime::Error) -> Self {
        CliError::from_wasm(error)
    }
}
