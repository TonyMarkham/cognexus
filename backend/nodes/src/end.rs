use crate::error::NodeError;

use cognexus_model::graph::{DataType, NodeDefinition, NodeDefinitionInfo};
use cognexus_types::SignalType;

use semver::Version;
use uuid::Uuid;

pub struct EndNode;

const ID: &str = "e7a20e26-27ce-4d49-9759-50db835d46e6";
const NAME: &str = "End";
const DESCRIPTION: &str = "Terminates workflow execution";

impl NodeDefinitionInfo for EndNode {
    fn definition_id(&self) -> Uuid {
        Uuid::parse_str(ID).unwrap()
    }

    fn name(&self) -> &str {
        NAME
    }

    fn description(&self) -> &str {
        DESCRIPTION
    }

    fn model_version(&self) -> Version {
        Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
    }

    fn input_port_specs(&self) -> Vec<(&str, Uuid)> {
        let signal_type = SignalType;
        vec![("signal", signal_type.type_id())]
    }

    fn output_port_specs(&self) -> Vec<(&str, Uuid)> {
        // End node has no outputs
        vec![]
    }
}

impl NodeDefinition for EndNode {
    type Error = NodeError;

    fn execute(&self, _inputs: Vec<u8>) -> Result<Vec<u8>, NodeError> {
        // End node consumes input and produces no output
        Ok(vec![])
    }
}
