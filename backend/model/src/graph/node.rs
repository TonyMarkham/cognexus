use uuid::Uuid;

pub struct Node {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) definition_id: Uuid,
}

impl Node {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn definition_id(&self) -> Uuid {
        self.definition_id
    }
}
