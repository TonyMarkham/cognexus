use std::any::Any;
use std::error::Error;

use semver::Version;
use uuid::Uuid;

/// Trait for defining data types that can flow through the graph.
/// Both first-party and plugin types implement this trait.
pub trait DataTypeInfo {
    /// Unique identifier for this data type.
    fn type_id(&self) -> Uuid;

    /// Human-readable name for this type.
    fn name(&self) -> &str;

    /// Description of this data type.
    fn description(&self) -> &str;

    /// Model version this type was built against.
    fn model_version(&self) -> Version;
}

pub trait DataType: DataTypeInfo {
    type Error: Error;

    /// Serialize a value of this type to bytes (for WASM boundary crossing).
    fn serialize(&self, value: Box<dyn Any>) -> Result<Vec<u8>, Self::Error>;

    /// Deserialize bytes back to a value of this type.
    fn deserialize(&self, bytes: &[u8]) -> Result<Box<dyn Any>, Self::Error>;
}
