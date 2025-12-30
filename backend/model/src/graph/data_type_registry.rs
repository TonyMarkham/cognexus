use crate::error::ModelError;
use crate::graph::DataTypeInfo;

use common::error::error_location::ErrorLocation;

use std::collections::HashMap;
use std::panic::Location;

use uuid::Uuid;

pub struct DataTypeRegistry {
    types: HashMap<Uuid, Box<dyn DataTypeInfo>>,
}

impl DataTypeRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    /// Register a data type
    #[track_caller]
    pub fn register<T>(&mut self, data_type: T) -> Result<(), ModelError>
    where
        T: DataTypeInfo + 'static,
    {
        let id = data_type.type_id();

        if self.types.contains_key(&id) {
            // TODO: Add logging when we have a logging system
            // log::warn!("Data type {} already registered", id);
            return Ok(());
        }

        self.types.insert(id, Box::new(data_type));
        Ok(())
    }

    #[track_caller]
    pub fn get(&self, type_id: &Uuid) -> Result<&dyn DataTypeInfo, ModelError> {
        self.types
            .get(type_id)
            .map(|boxed| boxed.as_ref())
            .ok_or_else(|| ModelError::ModelError {
                message: format!("Data type not found: {type_id}"),
                location: ErrorLocation::from(Location::caller()),
            })
    }
}
