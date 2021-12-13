use crate::ipc::DebugEntity;

pub struct History {
    history: Vec<DebugEntity>,
}

impl History {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, entity: DebugEntity) {
        self.history.push(entity)
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }
}

impl Default for History {
    fn default() -> Self {
        Self {
            history: Default::default(),
        }
    }
}
