//! Loader for WASM components using wasmtime.

use crate::error::CognexusError;

use common::error::error_location::ErrorLocation;

use std::panic::Location;
use std::path::Path;

use wasmtime::component::Component;
use wasmtime::{Config, Engine};

// Generate bindings for both plugin worlds
mod types_world {
    wasmtime::component::bindgen!({
        path: "../../../wit",
        world: "types-plugin",
    });
}

mod nodes_world {
    wasmtime::component::bindgen!({
        path: "../../../wit",
        world: "nodes-plugin",
    });
}

/// Loads and interrogates WASM components.
pub struct Loader {
    engine: Engine,
}

impl Loader {
    /// Create a new plugin loader with a configured wasmtime engine.
    #[track_caller]
    pub fn new() -> Result<Self, CognexusError> {
        let mut config = Config::default();
        config.wasm_component_model(true);

        let engine = Engine::new(&config).map_err(|e| CognexusError::CognexusError {
            message: format!("Failed to create wasmtime engine: {e}"),
            location: ErrorLocation::from(Location::caller()),
        })?;

        Ok(Self { engine })
    }

    /// Load a component from a file path.
    #[track_caller]
    pub fn load_component(&self, path: &Path) -> Result<Component, CognexusError> {
        Component::from_file(&self.engine, path).map_err(|e| CognexusError::CognexusError {
            message: format!("Failed to load component from {}: {}", path.display(), e),
            location: ErrorLocation::from(Location::caller()),
        })
    }
}