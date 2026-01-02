//! Plugin manager for discovering and loading WASM component plugins.

mod error;
mod loader;
mod scanner;
mod state;
mod translator;

pub use error::PluginManagerError;
pub use loader::Loader;
pub use scanner::scan_directory;
pub use state::State;

use common::error::error_location::ErrorLocation;

use std::panic::Location;
use std::path::PathBuf;

use log::{debug, info};

pub const TYPES_KIND: &str = "types";
pub const NODES_KIND: &str = "nodes";

/// Manages the plugin system lifecycle.
pub struct PluginManager {
    builtin_path: PathBuf,
    loader: Loader,
}

impl PluginManager {
    /// Create a new plugin manager with the specified builtin plugin directory.
    pub fn new(builtin_path: PathBuf) -> Result<Self, PluginManagerError> {
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
    pub fn discover_plugins(&mut self) -> Result<(), PluginManagerError> {
        // Scan for .wasm files
        let component_paths = scan_directory(&self.builtin_path)?;

        info!(
            "Found {} component(s) in {}",
            component_paths.len(),
            self.builtin_path.display()
        );

        for path in component_paths {
            debug!("Loading: {}", path.display());

            // Load the component
            let component = self.loader.load_component(&path)?;

            // Determine component type by introspecting its exports
            let kind = self.loader.determine_component_kind(&component)?;

            match kind {
                TYPES_KIND => {
                    let types = self.loader.discover_types(&component)?;
                    info!("Discovered {} type(s)", types.len());
                    for type_info in &types {
                        debug!("  Type: {} ({})", type_info.name, type_info.id);
                    }
                }
                NODES_KIND => {
                    let nodes = self.loader.discover_nodes(&component)?;
                    info!("Discovered {} node(s)", nodes.len());
                    for node_info in &nodes {
                        debug!("  Node: {} ({})", node_info.name, node_info.id);
                    }
                }
                // Defensive: determine_component_kind should only return TYPES_KIND or NODES_KIND
                _ => {
                    return Err(PluginManagerError::PluginError {
                        message: format!("Unknown component kind: {kind}"),
                        location: ErrorLocation::from(Location::caller()),
                        source: None,
                    });
                }
            }
        }

        Ok(())
    }
}
