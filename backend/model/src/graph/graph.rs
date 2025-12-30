use crate::error::ModelError;
use crate::graph::{Edge, EdgeBuilder, Node, NodeBuilder};

use common::error::error_location::ErrorLocation;

use std::panic::Location;

use uuid::Uuid;

pub struct Graph {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) nodes: Vec<Node>,
    pub(crate) edges: Vec<Edge>,
}

impl Graph {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// Add a node to the graph.
    /// If id is None, a new UUID will be generated.
    /// Returns the node's UUID.
    #[track_caller]
    pub fn add_node(
        &mut self,
        id: Option<Uuid>,
        name: &str,
        definition_id: Uuid,
    ) -> Result<Uuid, ModelError> {
        let mut builder = NodeBuilder::default();

        if let Some(id) = id {
            builder = builder.with_id(id);
        }

        let node = builder
            .with_name(name)
            .with_definition_id(definition_id)
            .build()?;

        let node_id = node.id();
        self.nodes.push(node);

        Ok(node_id)
    }

    /// Add an edge to the graph.
    /// If id is None, a new UUID will be generated.
    /// Validates that source and target nodes exist.
    /// Returns the edge's UUID.
    #[track_caller]
    pub fn add_edge(
        &mut self,
        id: Option<Uuid>,
        source_node_id: Uuid,
        source_port_id: Uuid,
        target_node_id: Uuid,
        target_port_id: Uuid,
    ) -> Result<Uuid, ModelError> {
        // Validate source node exists
        if !self.nodes.iter().any(|n| n.id() == source_node_id) {
            return Err(ModelError::ModelError {
                message: format!("Source node {} not found in graph", source_node_id),
                location: ErrorLocation::from(Location::caller()),
            });
        }

        // Validate target node exists
        if !self.nodes.iter().any(|n| n.id() == target_node_id) {
            return Err(ModelError::ModelError {
                message: format!("Target node {} not found in graph", target_node_id),
                location: ErrorLocation::from(Location::caller()),
            });
        }

        // TODO: Validate ports exist on nodes (requires registry access)

        let mut builder = EdgeBuilder::default();

        if let Some(id) = id {
            builder = builder.with_id(id);
        }

        let edge = builder
            .with_source_node_id(source_node_id)
            .with_source_port_id(source_port_id)
            .with_target_node_id(target_node_id)
            .with_target_port_id(target_port_id)
            .build()?;

        let edge_id = edge.id();
        self.edges.push(edge);

        Ok(edge_id)
    }
}
