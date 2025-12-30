use crate::error::NodeError;

use cognexus_model::graph::{DataTypeInfo, NodeDefinition, NodeDefinitionInfo, Port, PortBuilder};
use cognexus_types::SignalType;

use cognexus_model::error::ModelError;
use semver::Version;
use uuid::Uuid;

pub struct EndNode;

const ID: &str = "e7a20e26-27ce-4d49-9759-50db835d46e6";
const NAME: &str = "End";
const DESCRIPTION: &str = "Terminates workflow execution";
const INPUT_PORT_ID: &str = "f3500667-e80c-4483-b518-a3c71305d8ee";

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

    fn input_port_specs(&self) -> Result<Vec<Port>, ModelError> {
        let signal_type = SignalType;
        let port = PortBuilder::default()
            .with_id(Uuid::parse_str(INPUT_PORT_ID).unwrap())
            .with_name("signal")
            .with_data_type_id(signal_type.type_id())
            .build()?;

        Ok(vec![port])
    }

    fn output_port_specs(&self) -> Result<Vec<Port>, ModelError> {
        // End node has no outputs
        Ok(vec![])
    }
}

impl NodeDefinition for EndNode {
    type Error = NodeError;

    fn execute(&self, _inputs: Vec<u8>) -> Result<Vec<u8>, NodeError> {
        // End node consumes input and produces no output
        Ok(vec![])
    }
}
