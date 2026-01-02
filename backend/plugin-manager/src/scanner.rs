//! Scanner for discovering WASM component files in plugin directories.

use crate::error::PluginManagerError;

use common::error::error_location::ErrorLocation;

use std::fs;
use std::panic::Location;
use std::path::{Path, PathBuf};

const WASM_EXTENSION: &str = "wasm";

/// Scan a directory for .wasm component files.
///
/// Returns a list of paths to discovered component files.
/// Returns an error if the directory doesn't exist or can't be read.
pub fn scan_directory(dir: &Path) -> Result<Vec<PathBuf>, PluginManagerError> {
    let entries = fs::read_dir(dir).map_err(|e| PluginManagerError::IoError {
        message: format!("Failed to read plugin directory {}: {e}", dir.display()),
        location: ErrorLocation::from(Location::caller()),
        source: Some(Box::new(e)),
    })?;
    let mut components = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Only consider .wasm files
        if !path.is_file() {
            continue;
        }

        if path.extension().is_some_and(|ext| ext == WASM_EXTENSION) {
            components.push(path);
        }
    }

    Ok(components)
}
