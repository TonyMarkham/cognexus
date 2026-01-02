use common::error::error_location::ErrorLocation;

use std::io::Error;
use std::panic::Location;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum CognexusError {
    #[error("Cognexus Error: {message} {location}")]
    CognexusError {
        message: String,
        location: ErrorLocation,
    },

    #[error("IO Error: {message} {location}")]
    IoError {
        message: String,
        location: ErrorLocation,
        #[serde(skip)]
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Logger initialization error: {message} {location}")]
    LoggerInitialization {
        message: String,
        location: ErrorLocation,
        #[serde(skip)]
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Plugin manager error: {message} {location}")]
    PluginManagerError {
        message: String,
        location: ErrorLocation,
        #[serde(skip)]
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl CognexusError {
    #[track_caller]
    pub fn from_io(error: Error) -> Self {
        CognexusError::IoError {
            message: error.to_string(),
            location: ErrorLocation::from(Location::caller()),
            source: Some(Box::new(error)),
        }
    }

    #[track_caller]
    pub fn from_logger_init(error: log::SetLoggerError) -> Self {
        CognexusError::LoggerInitialization {
            message: error.to_string(),
            location: ErrorLocation::from(Location::caller()),
            source: Some(Box::new(error)),
        }
    }

    #[track_caller]
    fn from_plugin_manager(error: cognexus_plugin_manager::PluginManagerError) -> Self {
        CognexusError::PluginManagerError {
            message: error.to_string(),
            location: ErrorLocation::from(Location::caller()),
            source: Some(Box::new(error)),
        }
    }
}

impl From<Error> for CognexusError {
    #[track_caller]
    fn from(error: Error) -> Self {
        CognexusError::from_io(error)
    }
}

impl From<log::SetLoggerError> for CognexusError {
    #[track_caller]
    fn from(error: log::SetLoggerError) -> Self {
        CognexusError::from_logger_init(error)
    }
}

impl From<cognexus_plugin_manager::PluginManagerError> for CognexusError {
    #[track_caller]
    fn from(error: cognexus_plugin_manager::PluginManagerError) -> Self {
        CognexusError::from_plugin_manager(error)
    }
}
