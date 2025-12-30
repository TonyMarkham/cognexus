pub mod error;
mod signal;

pub use signal::SignalType;

// -------------------------------------------------------------------------- //

// Component Model bindings
use cognexus_model::graph::DataTypeInfo;
mod bindings;

use bindings::exports::cognexus::plugin::types::{Guest, TypeInfo};

struct Component;

impl Guest for Component {
    fn list_types() -> Vec<TypeInfo> {
        vec![TypeInfo {
            id: SignalType.type_id().to_string(),
            name: String::from(SignalType.name()),
            description: String::from(SignalType.description()),
            version: SignalType.model_version().to_string(),
        }]
    }
}

bindings::export!(Component with_types_in bindings);
