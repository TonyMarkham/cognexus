use common::error::error_location::ErrorLocation;

use thiserror::Error;

/// Errors that can occur during plugin management.
#[derive(Error, Debug)]
pub enum PluginManagerError {
    #[error("Plugin error: {message} {location}")]
    PluginError {
        message: String,
        location: ErrorLocation,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("IO error: {message} {location}")]
    IoError {
        message: String,
        location: ErrorLocation,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Wasmtime error: {message} {location}")]
    WasmtimeError {
        message: String,
        location: ErrorLocation,
        #[source]
        source: wasmtime::Error,
    },

    #[error("Lock error: {message}")]
    LockError {
        message: String,
        location: ErrorLocation,
    },
}

impl PluginManagerError {
    #[track_caller]
    pub fn from_io(error: std::io::Error) -> Self {
        PluginManagerError::IoError {
            message: error.to_string(),
            location: ErrorLocation::from(std::panic::Location::caller()),
            source: Some(Box::new(error)),
        }
    }

    #[track_caller]
    pub fn from_wasmtime(error: wasmtime::Error) -> Self {
        PluginManagerError::WasmtimeError {
            message: error.to_string(),
            location: ErrorLocation::from(std::panic::Location::caller()),
            source: error,
        }
    }
}

impl From<std::io::Error> for PluginManagerError {
    #[track_caller]
    fn from(error: std::io::Error) -> Self {
        PluginManagerError::from_io(error)
    }
}

impl From<wasmtime::Error> for PluginManagerError {
    #[track_caller]
    fn from(error: wasmtime::Error) -> Self {
        PluginManagerError::from_wasmtime(error)
    }
}
