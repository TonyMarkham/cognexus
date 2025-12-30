use crate::error::ModelError;
use crate::graph::Node;

use common::error::error_location::ErrorLocation;

use std::panic::Location;

use uuid::Uuid;

#[derive(Default)]
pub struct NodeBuilder {
    id: Option<Uuid>,
    name: Option<String>,
    definition_id: Option<Uuid>,
}

impl NodeBuilder {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self
    }

    pub fn with_definition_id(mut self, definition_id: Uuid) -> Self {
        self.definition_id = Some(definition_id);
        self
    }

    #[track_caller]
    pub fn build(self) -> Result<Node, ModelError> {
        let id = self.id.unwrap_or_else(Uuid::new_v4);

        let name = self.name.ok_or_else(|| ModelError::ModelError {
            message: String::from("Node name is required"),
            location: ErrorLocation::from(Location::caller()),
        })?;

        let definition_id = self.definition_id.ok_or_else(|| ModelError::ModelError {
            message: String::from("Node definition id is required"),
            location: ErrorLocation::from(Location::caller()),
        })?;

        Ok(Node {
            id,
            name,
            definition_id,
        })
    }
}
