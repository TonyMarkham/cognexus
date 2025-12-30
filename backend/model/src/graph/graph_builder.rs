use crate::error::ModelError;
use crate::graph::Graph;

use common::error::error_location::ErrorLocation;

use std::panic::Location;

use uuid::Uuid;

#[derive(Default)]
pub struct GraphBuilder {
    id: Option<Uuid>,
    name: Option<String>,
}

impl GraphBuilder {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self
    }

    #[track_caller]
    pub fn build(self) -> Result<Graph, ModelError> {
        let name = self.name.ok_or_else(|| ModelError::ModelError {
            message: String::from("Graph name is required"),
            location: ErrorLocation::from(Location::caller()),
        })?;

        let id = self.id.unwrap_or(Uuid::new_v4());

        Ok(Graph {
            id,
            name,
            nodes: Vec::new(),
            edges: Vec::new(),
        })
    }
}
