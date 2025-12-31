use common::error::error_location::ErrorLocation;

use std::io::Error;
use std::panic::Location;

use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum CognexusError {
    #[allow(dead_code)]
    #[error("Cognexus Error: {message} {location}")]
    CognexusError {
        message: String,
        location: ErrorLocation,
    },

    #[error("IO Error: {message} {location}")]
    IoError {
        message: String,
        location: ErrorLocation,
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
}

impl From<Error> for CognexusError {
    #[track_caller]
    fn from(error: Error) -> Self {
        CognexusError::from_io(error)
    }
}
