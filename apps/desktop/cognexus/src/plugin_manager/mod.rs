//! Plugin manager for discovering and loading WASM component plugins.
//!
//! This module handles:
//! - Scanning plugin directories for .wasm files
//! - Loading components using wasmtime
//! - Extracting metadata from components
//! - Populating type and node registries

mod loader;
mod scanner;
mod state;

pub use loader::Loader;
pub use scanner::Scanner;
pub use state::State;

use crate::error::CognexusError;

use std::path::PathBuf;

/// Manages the plugin system lifecycle.
pub struct PluginManager {
    builtin_path: PathBuf,
    loader: Loader,
}

impl PluginManager {
    /// Create a new plugin manager with the specified builtin plugin directory.
    pub fn new(builtin_path: PathBuf) -> Result<Self, CognexusError> {
        let loader = Loader::new()?;

        Ok(Self {
            builtin_path,
            loader,
        })
    }
}
