use std::slice::Iter;

use crate::ipc::DebugEntity;

pub struct History {
    history: Vec<DebugEntity>,
    dirty: bool,
    prev_clean: usize,
}

impl History {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, entity: DebugEntity) {
        self.dirty = true;
        self.history.push(entity);
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }

    pub fn clean(&mut self) {
        self.dirty = false;
        self.prev_clean = self.len();
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn dirty_entities(&mut self) -> std::slice::IterMut<'_, DebugEntity> {
        self.history[self.prev_clean..].iter_mut()
    }
}

impl Default for History {
    fn default() -> Self {
        Self {
            history: Default::default(),
            dirty: false,
            prev_clean: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use parry3d::bounding_volume::AABB;

    use crate::ipc::parry::ParryDebugEntityType;

    use super::*;

    #[test]
    fn test_dirty() {
        let mut history = History::new();
        let aabb = AABB::new_invalid();
        let entity = ParryDebugEntityType::new_aabb_entity(aabb);
        history.push(entity.clone());

        assert!(history.is_dirty());
        assert_eq!(history.dirty_entities().len(), 1);

        history.push(entity.clone());

        assert_eq!(history.dirty_entities().len(), 2);

        history.clean();
        assert!(!history.is_dirty());

        history.push(entity.clone());

        assert!(history.is_dirty());
        assert_eq!(history.dirty_entities().len(), 1);

        history.push(entity.clone());
        assert_eq!(history.dirty_entities().len(), 2);
    }
}
