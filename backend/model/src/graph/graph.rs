use crate::error::ModelError;
use crate::graph::{Edge, EdgeBuilder, Node, NodeBuilder, DataTypeRegistry, NodeDefinitionRegistry};

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
        registry: &NodeDefinitionRegistry,
        id: Option<Uuid>,
        name: &str,
        definition_id: Uuid,
    ) -> Result<Uuid, ModelError> {
        // Validate definition exists in registry
        registry.get(&definition_id)?;

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
    /// Validates that source and target nodes exist and ports are valid.
    /// Returns the edge's UUID.
    #[track_caller]
    pub fn add_edge(
        &mut self,
        registry: &NodeDefinitionRegistry,
        id: Option<Uuid>,
        source_node_id: Uuid,
        source_port_id: Uuid,
        target_node_id: Uuid,
        target_port_id: Uuid,
    ) -> Result<Uuid, ModelError> {
        // Validate source node exists
        let source_node = self
            .nodes
            .iter()
            .find(|n| n.id() == source_node_id)
            .ok_or_else(|| ModelError::ModelError {
                message: format!("Source node {source_node_id} not found in graph"),
                location: ErrorLocation::from(Location::caller()),
            })?;

        // Validate target node exists
        let target_node = self
            .nodes
            .iter()
            .find(|n| n.id() == target_node_id)
            .ok_or_else(|| ModelError::ModelError {
                message: format!("Target node {target_node_id} not found in graph"),
                location: ErrorLocation::from(Location::caller()),
            })?;

        // Validate source port exists on source node definition
        let source_definition = registry.get(&source_node.definition_id())?;
        let source_ports = source_definition.output_port_specs()?;
        if !source_ports.iter().any(|p| p.id() == source_port_id) {
            return Err(ModelError::ModelError {
                message: format!("Source port {source_port_id} not found on node {source_node_id}"),
                location: ErrorLocation::from(Location::caller()),
            });
        }

        // Validate target port exists on target node definition
        let target_definition = registry.get(&target_node.definition_id())?;
        let target_ports = target_definition.input_port_specs()?;
        if !target_ports.iter().any(|p| p.id() == target_port_id) {
            return Err(ModelError::ModelError {
                message: format!("Target port {target_port_id} not found on node {target_node_id}"),
                location: ErrorLocation::from(Location::caller()),
            });
        }

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
