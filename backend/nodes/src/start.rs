use crate::error::NodeError;

use cognexus_model::graph::{DataType, NodeDefinition, NodeDefinitionInfo, Port, PortBuilder};
use cognexus_types::SignalType;

use cognexus_model::error::ModelError;
use semver::Version;
use uuid::Uuid;

pub struct StartNode;

const ID: &str = "40ebe0be-d2db-4eed-80f3-91267352ee42";
const NAME: &str = "Start";
const DESCRIPTION: &str = "Initiates workflow execution";
const OUTPUT_PORT_ID: &str = "a7c33534-0b4c-4311-9891-2cbf609dbd12";

impl NodeDefinitionInfo for StartNode {
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
        // Start node has no inputs
        Ok(vec![])
    }

    fn output_port_specs(&self) -> Result<Vec<Port>, ModelError> {
        let signal_type = SignalType;
        let port = PortBuilder::default()
            .with_id(Uuid::parse_str(OUTPUT_PORT_ID).unwrap())
            .with_name("signal")
            .with_data_type_id(signal_type.type_id())
            .build()?;

        Ok(vec![port])
    }
}

impl NodeDefinition for StartNode {
    type Error = NodeError;

    fn execute(&self, _inputs: Vec<u8>) -> Result<Vec<u8>, NodeError> {
        Ok(vec![])
    }
}
