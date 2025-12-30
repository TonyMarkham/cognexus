use crate::error::NodeError;

use cognexus_model::graph::{DataType, NodeDefinition, NodeDefinitionInfo};
use cognexus_types::SignalType;

use semver::Version;
use uuid::Uuid;

pub struct StartNode;

const ID: &str = "40ebe0be-d2db-4eed-80f3-91267352ee42";
const NAME: &str = "Start";
const DESCRIPTION: &str = "Initiates workflow execution";

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

    fn input_port_specs(&self) -> Vec<(&str, Uuid)> {
        // Start node has no inputs
        vec![]
    }

    fn output_port_specs(&self) -> Vec<(&str, Uuid)> {
        let signal_type = SignalType;
        vec![("signal", signal_type.type_id())]
    }
}

impl NodeDefinition for StartNode {
    type Error = NodeError;

    fn execute(&self, _inputs: Vec<u8>) -> Result<Vec<u8>, NodeError> {
        Ok(vec![])
    }
}
