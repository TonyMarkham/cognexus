use crate::error::ModelError;
use crate::graph::NodeDefinitionInfo;

use common::error::error_location::ErrorLocation;

use std::collections::HashMap;
use std::panic::Location;

use uuid::Uuid;

pub struct NodeDefinitionRegistry {
    definitions: HashMap<Uuid, Box<dyn NodeDefinitionInfo>>,
}

impl NodeDefinitionRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }

    /// Register a node definition
    #[track_caller]
    pub fn register<T>(&mut self, definition: T) -> Result<(), ModelError>
    where
        T: NodeDefinitionInfo + 'static,
    {
        let id = definition.definition_id();

        if self.definitions.contains_key(&id) {
            // TODO: Add logging when we have a logging system
            // log::warn!("Node definition {} already registered", id);
            return Ok(());
        }

        self.definitions.insert(id, Box::new(definition));
        Ok(())
    }

    #[track_caller]
    pub fn get(&self, definition_id: &Uuid) -> Result<&dyn NodeDefinitionInfo, ModelError> {
        self.definitions
            .get(definition_id)
            .map(|boxed| boxed.as_ref())
            .ok_or_else(|| ModelError::ModelError {
                message: format!("Node definition not found: {definition_id} "),
                location: ErrorLocation::from(Location::caller()),
            })
    }
}
