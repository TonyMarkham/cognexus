//! Translation layer between WIT plugin interfaces and Protobuf messages.

use crate::loader::{nodes_world, types_world};

// Import generated protobuf types
use proto::{Direction, NodeDefinition, PortSpec, TypeDefinition};

/// Convert WIT TypeInfo to Protobuf TypeDefinition
pub fn wit_type_to_proto(
    wit: types_world::exports::cognexus::plugin::types::TypeInfo,
) -> TypeDefinition {
    TypeDefinition {
        id: wit.id,
        name: wit.name,
        description: wit.description,
        version: wit.version,
    }
}

/// Convert WIT NodeInfo to Protobuf NodeDefinition
pub fn wit_node_to_proto(
    wit: nodes_world::exports::cognexus::plugin::nodes::NodeInfo,
) -> NodeDefinition {
    NodeDefinition {
        id: wit.id,
        name: wit.name,
        description: wit.description,
        version: wit.version,
        input_ports: wit.input_ports.into_iter().map(wit_port_to_proto).collect(),
        output_ports: wit
            .output_ports
            .into_iter()
            .map(wit_port_to_proto)
            .collect(),
    }
}

/// Convert WIT PortSpec to Protobuf PortSpec
fn wit_port_to_proto(wit: nodes_world::exports::cognexus::plugin::nodes::PortSpec) -> PortSpec {
    PortSpec {
        id: wit.id,
        name: wit.name,
        direction: wit_direction_to_proto(wit.direction) as i32,
        data_type_id: wit.data_type_id,
    }
}

/// Convert WIT Direction to Protobuf Direction
fn wit_direction_to_proto(
    wit: nodes_world::exports::cognexus::plugin::nodes::Direction,
) -> Direction {
    match wit {
        nodes_world::exports::cognexus::plugin::nodes::Direction::Input => Direction::Input,
        nodes_world::exports::cognexus::plugin::nodes::Direction::Output => Direction::Output,
    }
}
