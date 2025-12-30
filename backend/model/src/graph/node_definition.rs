use crate::graph::Port;

use std::error::Error;

use crate::error::ModelError;
use semver::Version;
use uuid::Uuid;

/// Registry-safe trait containing only metadata methods (no execute).
/// Used for storing definitions in the registry without error type issues.
pub trait NodeDefinitionInfo {
    /// Unique identifier for this node definition (the node type).
    fn definition_id(&self) -> Uuid;

    /// Human-readable name for this node type.
    fn name(&self) -> &str;

    /// Description of what this node does.
    fn description(&self) -> &str;

    /// Model version this node definition was built against.
    fn model_version(&self) -> Version;

    /// Specifications for input ports: (name, data_type_id).
    fn input_port_specs(&self) -> Result<Vec<Port>, ModelError>;

    /// Specifications for output ports: (name, data_type_id).
    fn output_port_specs(&self) -> Result<Vec<Port>, ModelError>;
}

/// Trait for defining node types that can be instantiated in the graph.
/// Both first-party and plugin nodes implement this trait.
pub trait NodeDefinition: NodeDefinitionInfo {
    /// The error type for execution operations.
    type Error: Error;

    /// Execute this node with the given inputs (serialized as bytes for WASM compatibility).
    /// Returns serialized outputs.
    fn execute(&self, inputs: Vec<u8>) -> Result<Vec<u8>, Self::Error>;
}
