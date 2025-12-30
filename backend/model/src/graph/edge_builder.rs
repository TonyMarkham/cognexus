use crate::error::ModelError;
use crate::graph::Edge;

use common::error::error_location::ErrorLocation;

use std::panic::Location;

use uuid::Uuid;

#[derive(Default)]
pub struct EdgeBuilder {
    id: Option<Uuid>,
    source_node_id: Option<Uuid>,
    source_port_id: Option<Uuid>,
    target_node_id: Option<Uuid>,
    target_port_id: Option<Uuid>,
}

impl EdgeBuilder {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_source_node_id(mut self, source_node_id: Uuid) -> Self {
        self.source_node_id = Some(source_node_id);
        self
    }

    pub fn with_source_port_id(mut self, source_port_id: Uuid) -> Self {
        self.source_port_id = Some(source_port_id);
        self
    }

    pub fn with_target_node_id(mut self, target_node_id: Uuid) -> Self {
        self.target_node_id = Some(target_node_id);
        self
    }

    pub fn with_target_port_id(mut self, target_port_id: Uuid) -> Self {
        self.target_port_id = Some(target_port_id);
        self
    }

    #[track_caller]
    pub fn build(self) -> Result<Edge, ModelError> {
        let id = self.id.unwrap_or_else(Uuid::new_v4);

        let source_node_id = self.source_node_id.ok_or_else(|| ModelError::ModelError {
            message: String::from("Edge source_node_id is required"),
            location: ErrorLocation::from(Location::caller()),
        })?;

        let source_port_id = self.source_port_id.ok_or_else(|| ModelError::ModelError {
            message: String::from("Edge source_port_id is required"),
            location: ErrorLocation::from(Location::caller()),
        })?;

        let target_node_id = self.target_node_id.ok_or_else(|| ModelError::ModelError {
            message: String::from("Edge target_node_id is required"),
            location: ErrorLocation::from(Location::caller()),
        })?;

        let target_port_id = self.target_port_id.ok_or_else(|| ModelError::ModelError {
            message: String::from("Edge target_port_id is required"),
            location: ErrorLocation::from(Location::caller()),
        })?;

        Ok(Edge {
            id,
            source_node_id,
            source_port_id,
            target_node_id,
            target_port_id,
        })
    }
}
