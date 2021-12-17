use std::{collections::HashMap, slice::IterMut};

use threebug_core::{ipc::DebugEntity, Entity};

use crate::ui::EntityUiState;

#[derive(Default)]
pub struct Entities {
    pub entities: Vec<DebugEntity>,
    pub ui: HashMap<Entity, EntityUiState>,
    dirty: bool,
    prev_clean: usize,
}

impl Entities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, entity: DebugEntity) {
        self.dirty = true;
        self.ui.insert(entity.id, EntityUiState::new(entity.id));
        self.entities.push(entity);
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn clean(&mut self) {
        self.dirty = false;
        self.prev_clean = self.len();
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn dirty_entities(&mut self) -> IterMut<'_, DebugEntity> {
        self.entities[self.prev_clean..].iter_mut()
    }

    pub fn entities_mut(&mut self) -> impl Iterator<Item = &mut DebugEntity> {
        self.entities.iter_mut()
    }

    pub fn entities(&self) -> impl Iterator<Item = &DebugEntity> {
        self.entities.iter()
    }
}

#[cfg(test)]
mod tests {
    use parry3d::bounding_volume::AABB;

    use threebug_core::ipc::parry::ParryDebugEntityType;

    use super::*;

    #[test]
    fn test_dirty() {
        let mut history = Entities::new();
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

        history.push(entity);
        assert_eq!(history.dirty_entities().len(), 2);
    }
}
