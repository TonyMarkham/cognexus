use crate::error::TypeError;

use cognexus_model::graph::{DataType, DataTypeInfo};

use common::error::error_location::ErrorLocation;

use std::any::Any;
use std::panic::Location;

use semver::Version;
use uuid::Uuid;

pub struct SignalType;

const ID: &str = "989bcbb2-b1a1-4f3f-be15-22ada278aedc";
const NAME: &str = "Signal";
const DESCRIPTION: &str = "A flow control signal with no data payload";

impl DataTypeInfo for SignalType {
    fn type_id(&self) -> Uuid {
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
}

impl DataType for SignalType {
    type Error = TypeError;

    fn serialize(&self, _value: Box<dyn Any>) -> Result<Vec<u8>, TypeError> {
        // Signal has no data, return empty bytes
        Ok(vec![])
    }

    #[track_caller]
    fn deserialize(&self, bytes: &[u8]) -> Result<Box<dyn Any>, TypeError> {
        if !bytes.is_empty() {
            return Err(TypeError::DeserializationError {
                message: String::from("Signal type expects empty bytes."),
                location: ErrorLocation::from(Location::caller()),
            });
        }

        Ok(Box::new(()))
    }
}
