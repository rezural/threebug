use serde::*;

pub mod ipc;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entity {
    pub id: u64,
}

impl Entity {
    // new entities are created as invalid entities, when deserialized from the client
    const INVALID_ID: u64 = u64::MAX;
    pub fn invalid() -> Self {
        Self::default()
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            id: Self::INVALID_ID,
        }
    }
}

impl From<u64> for Entity {
    fn from(bits: u64) -> Self {
        Self { id: bits }
    }
}

pub struct EntityRegistry {
    current: u64,
}

//FIXME: use atomics?
impl EntityRegistry {
    /// Get a new entity ID, and increment the current counter
    pub fn new_id(&mut self) -> u64 {
        self.current += 1;
        self.current - 1
    }

    pub fn assign_id(&mut self, entity: &mut Entity) {
        let new_id = self.new_id();
        entity.id = new_id;
    }
}

impl Default for EntityRegistry {
    fn default() -> Self {
        Self { current: 0 }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
