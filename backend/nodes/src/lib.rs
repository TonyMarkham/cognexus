mod end;
pub mod error;
mod start;

pub use end::EndNode;
pub use start::StartNode;

// -------------------------------------------------------------------------- //

// Component Model bindings
mod bindings;

use bindings::exports::cognexus::plugin::nodes::{Direction, Guest, NodeInfo, PortSpec};
use cognexus_model::graph::NodeDefinitionInfo;

struct Component;

impl Guest for Component {
    fn list_nodes() -> Vec<NodeInfo> {
        let start_node = StartNode;
        let end_node = EndNode;

        vec![
            // StartNode
            NodeInfo {
                id: start_node.definition_id().to_string(),
                name: start_node.name().to_string(),
                description: start_node.description().to_string(),
                version: start_node.model_version().to_string(),
                input_ports: vec![],
                output_ports: start_node
                    .output_port_specs()
                    .unwrap_or_default()
                    .iter()
                    .map(|port| PortSpec {
                        id: port.id().to_string(),
                        name: port.name().to_string(),
                        direction: Direction::Output,
                        data_type_id: port.data_type_id().to_string(),
                    })
                    .collect(),
            },
            // EndNode
            NodeInfo {
                id: end_node.definition_id().to_string(),
                name: end_node.name().to_string(),
                description: end_node.description().to_string(),
                version: end_node.model_version().to_string(),
                input_ports: end_node
                    .input_port_specs()
                    .unwrap_or_default()
                    .iter()
                    .map(|port| PortSpec {
                        id: port.id().to_string(),
                        name: port.name().to_string(),
                        direction: Direction::Input,
                        data_type_id: port.data_type_id().to_string(),
                    })
                    .collect(),
                output_ports: vec![],
            },
        ]
    }
}

bindings::export!(Component with_types_in bindings);
