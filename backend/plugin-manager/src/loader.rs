//! Loader for WASM components using wasmtime.

use crate::State;
use crate::error::PluginManagerError;
use crate::{NODES_KIND, TYPES_KIND};

use common::error::error_location::ErrorLocation;

use std::panic::Location;
use std::path::Path;

use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::p2;

// WIT interface identifiers
const TYPES_INTERFACE: &str = "cognexus:plugin/types";
const NODES_INTERFACE: &str = "cognexus:plugin/nodes";

// Generate bindings for both plugin worlds
pub mod types_world {
    wasmtime::component::bindgen!({
        path: "../../wit",
        world: "types-plugin",
    });
}

pub mod nodes_world {
    wasmtime::component::bindgen!({
        path: "../../wit",
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
    pub fn new() -> Result<Self, PluginManagerError> {
        let mut config = Config::default();
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;

        Ok(Self { engine })
    }

    /// Load a component from a file path.
    #[track_caller]
    pub fn load_component(&self, path: &Path) -> Result<Component, PluginManagerError> {
        Component::from_file(&self.engine, path).map_err(PluginManagerError::from_wasmtime)
    }

    /// Generic discovery helper that sets up WASI, instantiates a plugin, and calls a discovery function.
    fn discover<T, F>(&self, call_fn: F) -> Result<T, PluginManagerError>
    where
        F: FnOnce(&mut Store<State>, &Linker<State>) -> Result<T, wasmtime::Error>,
    {
        // Create linker with WASI support
        let mut linker = Linker::<State>::new(&self.engine);
        p2::add_to_linker_sync(&mut linker)?;

        // Create store with state
        let state = State::default();
        let mut store = Store::new(&self.engine, state);

        // Call the provided discovery function with store and linker
        call_fn(&mut store, &linker).map_err(PluginManagerError::from_wasmtime)
    }

    /// Discover data types from a types-plugin component.
    #[track_caller]
    pub fn discover_types(
        &self,
        component: &Component,
    ) -> Result<Vec<types_world::exports::cognexus::plugin::types::TypeInfo>, PluginManagerError>
    {
        self.discover(|store, linker| {
            let plugin = types_world::TypesPlugin::instantiate(&mut *store, component, linker)?;
            plugin.cognexus_plugin_types().call_list_types(&mut *store)
        })
    }

    /// Discover nodes from a nodes-plugin component.
    #[track_caller]
    pub fn discover_nodes(
        &self,
        component: &Component,
    ) -> Result<Vec<nodes_world::exports::cognexus::plugin::nodes::NodeInfo>, PluginManagerError>
    {
        self.discover(|store, linker| {
            let plugin = nodes_world::NodesPlugin::instantiate(&mut *store, component, linker)?;
            plugin.cognexus_plugin_nodes().call_list_nodes(&mut *store)
        })
    }

    /// Determine what kind of plugin a component is by examining its exports.
    ///
    /// Returns "types" if it exports the cognexus:plugin/types interface,
    /// "nodes" if it exports cognexus:plugin/nodes, or an error if neither.
    #[track_caller]
    pub fn determine_component_kind(
        &self,
        component: &Component,
    ) -> Result<&'static str, PluginManagerError> {
        let component_type = component.component_type();

        for (name, _item) in component_type.exports(&self.engine) {
            if name == TYPES_INTERFACE {
                return Ok(TYPES_KIND);
            }
            if name == NODES_INTERFACE {
                return Ok(NODES_KIND);
            }
        }

        Err(PluginManagerError::PluginError {
            message: format!(
                "Component does not export {TYPES_INTERFACE} or {NODES_INTERFACE} interface"
            ),
            location: ErrorLocation::from(Location::caller()),
            source: None,
        })
    }
}
