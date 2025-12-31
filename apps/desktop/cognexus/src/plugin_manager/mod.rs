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

    /// Discover and load all plugins from the builtin directory.
    ///
    /// This scans for .wasm files, loads each component, determines its type
    /// by introspecting exports, and calls the appropriate discovery function.
    pub fn discover_plugins(&mut self) -> Result<(), CognexusError> {
        // Scan for .wasm files
        let component_paths = Scanner::scan_directory(&self.builtin_path)?;

        println!(
            "Found {} component(s) in {}",
            component_paths.len(),
            self.builtin_path.display()
        );

        for path in component_paths {
            println!("  Loading: {}", path.display());

            // Load the component
            let component = self.loader.load_component(&path)?;

            // Determine component type by introspecting its exports
            let kind = self.loader.determine_component_kind(&component)?;

            match kind {
                "types" => {
                    let types = self.loader.discover_types(&component)?;
                    println!("    Discovered {} type(s)", types.len());
                    for type_info in types {
                        println!("      - {} ({})", type_info.name, type_info.id);
                    }
                }
                "nodes" => {
                    let nodes = self.loader.discover_nodes(&component)?;
                    println!("    Discovered {} node(s)", nodes.len());
                    for node_info in nodes {
                        println!("      - {} ({})", node_info.name, node_info.id);
                    }
                }
                _ => {
                    return Err(CognexusError::CognexusError {
                        message: format!("Unknown component kind: {kind}"),
                        location: common::error::error_location::ErrorLocation::from(
                            std::panic::Location::caller(),
                        ),
                    });
                }
            }
        }

        Ok(())
    }
}
