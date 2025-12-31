//! Scanner for discovering WASM component files in plugin directories.

use crate::error::CognexusError;

use common::error::error_location::ErrorLocation;

use std::fs;
use std::panic::Location;
use std::path::{Path, PathBuf};

/// Scans directories for WASM component files.
pub struct Scanner;

impl Scanner {
    /// Scan a directory for .wasm files.
    ///
    /// Returns a list of paths to discovered component files.
    /// Non-existent directories or read errors are logged but don't fail the scan.
    pub fn scan_directory(dir: &Path) -> Result<Vec<PathBuf>, CognexusError> {
        let mut components = Vec::new();

        // Check if directory exists
        if !dir.exists() {
            return Err(CognexusError::CognexusError {
                message: format!("Plugin directory does not exist: {}", dir.display()),
                location: ErrorLocation::from(Location::caller()),
            });
        }

        // Read directory entries
        let entries = fs::read_dir(dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // Only consider files with .wasm extension
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "wasm" {
                        components.push(path);
                    }
                }
            }
        }

        Ok(components)
    }
}
