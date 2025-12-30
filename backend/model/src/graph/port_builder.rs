use crate::error::ModelError;
use crate::graph::Port;

use common::error::error_location::ErrorLocation;

use std::panic::Location;

use uuid::Uuid;

#[derive(Default)]
pub struct PortBuilder {
    id: Option<Uuid>,
    name: Option<String>,
    data_type_id: Option<Uuid>,
}

impl PortBuilder {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self
    }

    pub fn with_data_type_id(mut self, data_type_id: Uuid) -> Self {
        self.data_type_id = Some(data_type_id);
        self
    }

    #[track_caller]
    pub fn build(self) -> Result<Port, ModelError> {
        let id = self.id.unwrap_or_else(|| Uuid::new_v4());

        let name = self.name.ok_or_else(|| ModelError::PortError {
            message: String::from("Port name is required"),
            port_name: String::from("<unnamed>"),
            data_type_id: self.data_type_id.unwrap_or_else(Uuid::nil),
            location: ErrorLocation::from(Location::caller()),
        })?;

        let data_type_id = self.data_type_id.ok_or_else(|| ModelError::PortError {
            message: String::from("Port Data Type ID is required"),
            port_name: name.clone(),
            data_type_id: Uuid::nil(),
            location: ErrorLocation::from(Location::caller()),
        })?;

        Ok(Port {
            id,
            name,
            data_type_id,
        })
    }
}
