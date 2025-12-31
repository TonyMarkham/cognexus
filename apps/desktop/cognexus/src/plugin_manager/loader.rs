//! Loader for WASM components using wasmtime.

use crate::error::CognexusError;
use crate::plugin_manager::State;

use common::error::error_location::ErrorLocation;

use std::panic::Location;
use std::path::Path;

use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::p2;

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

    /// Discover data types from a types-plugin component.
    #[track_caller]
    pub fn discover_types(
        &self,
        component: &Component,
    ) -> Result<Vec<types_world::exports::cognexus::plugin::types::TypeInfo>, CognexusError> {
        // Create linker with WASI support
        let mut linker = Linker::new(&self.engine);
        p2::add_to_linker_sync(&mut linker).map_err(|e| CognexusError::CognexusError {
            message: format!("Failed to add WASI to linker: {}", e),
            location: ErrorLocation::from(Location::caller()),
        })?;

        // Create store with state
        let state = State::new();
        let mut store = Store::new(&self.engine, state);

        // Instantiate and call discovery function
        let plugin = types_world::TypesPlugin::instantiate(&mut store, component, &linker)
            .map_err(|e| CognexusError::CognexusError {
                message: format!("Failed to instantiate types plugin: {e}"),
                location: ErrorLocation::from(Location::caller()),
            })?;

        let types = plugin
            .cognexus_plugin_types()
            .call_list_types(&mut store)
            .map_err(|e| CognexusError::CognexusError {
                message: format!("Failed to call list_types: {e}"),
                location: ErrorLocation::from(Location::caller()),
            })?;

        Ok(types)
    }

    /// Discover nodes from a nodes-plugin component.
    #[track_caller]
    pub fn discover_nodes(
        &self,
        component: &Component,
    ) -> Result<Vec<nodes_world::exports::cognexus::plugin::nodes::NodeInfo>, CognexusError> {
        // Create linker with WASI support
        let mut linker = Linker::new(&self.engine);
        p2::add_to_linker_sync(&mut linker).map_err(|e| CognexusError::CognexusError {
            message: format!("Failed to add WASI to linker: {e}"),
            location: ErrorLocation::from(Location::caller()),
        })?;

        // Create store with state
        let state = State::new();
        let mut store = Store::new(&self.engine, state);

        // Instantiate and call discovery function
        let plugin = nodes_world::NodesPlugin::instantiate(&mut store, component, &linker)
            .map_err(|e| CognexusError::CognexusError {
                message: format!("Failed to instantiate nodes plugin: {e}"),
                location: ErrorLocation::from(Location::caller()),
            })?;

        let nodes = plugin
            .cognexus_plugin_nodes()
            .call_list_nodes(&mut store)
            .map_err(|e| CognexusError::CognexusError {
                message: format!("Failed to call list_nodes: {e}"),
                location: ErrorLocation::from(Location::caller()),
            })?;

        Ok(nodes)
    }
}
