use uuid::Uuid;

pub struct Port {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) data_type_id: Uuid,
}

impl Port {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type_id(&self) -> Uuid {
        self.data_type_id
    }
}
