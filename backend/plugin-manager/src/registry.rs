//! In-memory registry for storing discovered plugin metadata.

use crate::error::PluginManagerError;

use common::error::error_location::ErrorLocation;

use proto::{NodeDefinition, TypeDefinition};

use std::collections::HashMap;
use std::panic::Location;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use log::{debug, warn};

/// Thread-safe registry for plugin metadata.
///
/// Stores discovered nodes and types in memory, providing concurrent
/// read access and exclusive write access for registration.
///
/// Lock poisoning is handled gracefully by attempting recovery.
#[derive(Default)]
struct RegistryInner {
    nodes: HashMap<String, NodeDefinition>,
    types: HashMap<String, TypeDefinition>,
}

#[derive(Clone, Default)]
pub struct Registry {
    inner: Arc<RwLock<RegistryInner>>,
}

impl Registry {
    /// Register a node definition.
    ///
    /// If a node with the same ID already exists, it will be replaced
    /// and a warning will be logged.
    ///
    /// # Errors
    ///
    /// Returns `PluginManagerError::LockError` if the registry lock is poisoned.
    #[track_caller]
    pub fn register_node(&self, node: NodeDefinition) -> Result<(), PluginManagerError> {
        let mut inner = self.write_lock()?;

        if let Some(existing) = inner.nodes.get(&node.id) {
            warn!(
                "Replacing existing node '{}' (version {}) with version {}",
                node.id, existing.version, node.version
            );
        } else {
            debug!("Registering node: {} ({})", node.name, node.id);
        }

        inner.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// Register a type definition.
    ///
    /// If a type with the same ID already exists, it will be replaced
    /// and a warning will be logged.
    ///
    /// # Errors
    ///
    /// Returns `PluginManagerError::LockError` if the registry lock is poisoned.
    #[track_caller]
    pub fn register_type(&self, type_def: TypeDefinition) -> Result<(), PluginManagerError> {
        let mut inner = self.write_lock()?;

        if let Some(existing) = inner.types.get(&type_def.id) {
            warn!(
                "Replacing existing type '{}' (version {}) with version {}",
                type_def.id, existing.version, type_def.version
            );
        } else {
            debug!("Registering type: {} ({})", type_def.name, type_def.id);
        }

        inner.types.insert(type_def.id.clone(), type_def);
        Ok(())
    }

    /// Get a node definition by ID.
    ///
    /// Returns `None` if the node is not registered.
    ///
    /// # Errors
    ///
    /// Returns `PluginManagerError::LockError` if the registry lock is poisoned.
    #[track_caller]
    pub fn get_node(&self, id: &str) -> Result<Option<NodeDefinition>, PluginManagerError> {
        let inner = self.read_lock()?;
        Ok(inner.nodes.get(id).cloned())
    }

    /// Get a type definition by ID.
    ///
    /// Returns `None` if the type is not registered.
    ///
    /// # Errors
    ///
    /// Returns `PluginManagerError::LockError` if the registry lock is poisoned.
    #[track_caller]
    pub fn get_type(&self, id: &str) -> Result<Option<TypeDefinition>, PluginManagerError> {
        let inner = self.read_lock()?;
        Ok(inner.types.get(id).cloned())
    }

    /// List all registered node definitions.
    ///
    /// # Errors
    ///
    /// Returns `PluginManagerError::LockError` if the registry lock is poisoned.
    #[track_caller]
    pub fn list_nodes(&self) -> Result<Vec<NodeDefinition>, PluginManagerError> {
        let inner = self.read_lock()?;
        Ok(inner.nodes.values().cloned().collect())
    }

    /// List all registered type definitions.
    ///
    /// # Errors
    ///
    /// Returns `PluginManagerError::LockError` if the registry lock is poisoned.
    #[track_caller]
    pub fn list_types(&self) -> Result<Vec<TypeDefinition>, PluginManagerError> {
        let inner = self.read_lock()?;
        Ok(inner.types.values().cloned().collect())
    }

    /// Acquire a read lock, treating poison errors as failures.
    #[track_caller]
    fn read_lock(&self) -> Result<RwLockReadGuard<'_, RegistryInner>, PluginManagerError> {
        self.inner.read().map_err(|e| {
            warn!("Registry read lock was poisoned");
            PluginManagerError::LockError {
                message: format!("Failed to acquire registry read lock: {e}"),
                location: ErrorLocation::from(Location::caller()),
            }
        })
    }

    /// Acquire a write lock, treating poison errors as failures.
    #[track_caller]
    fn write_lock(&self) -> Result<RwLockWriteGuard<'_, RegistryInner>, PluginManagerError> {
        self.inner.write().map_err(|e| {
            warn!("Registry write lock was poisoned");
            PluginManagerError::LockError {
                message: format!("Failed to acquire registry write lock: {e}"),
                location: ErrorLocation::from(Location::caller()),
            }
        })
    }
}
