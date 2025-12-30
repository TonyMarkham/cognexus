use uuid::Uuid;

pub struct Edge {
    pub(crate) id: Uuid,
    pub(crate) source_node_id: Uuid,
    pub(crate) source_port_id: Uuid,
    pub(crate) target_node_id: Uuid,
    pub(crate) target_port_id: Uuid,
}

impl Edge {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn source_node_id(&self) -> Uuid {
        self.source_node_id
    }

    pub fn source_port_id(&self) -> Uuid {
        self.source_port_id
    }

    pub fn target_node_id(&self) -> Uuid {
        self.target_node_id
    }

    pub fn target_port_id(&self) -> Uuid {
        self.target_port_id
    }
}
