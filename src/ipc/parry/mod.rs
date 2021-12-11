use parry3d::bounding_volume;
use serde::*;

use super::{DebugEntity, DebugEntityType};

#[derive(Debug, Serialize, Deserialize)]
pub enum ParryDebugEntityType {
    AABB { aabb: bounding_volume::AABB },
}

impl ParryDebugEntityType {
    pub fn new_aabb_entity(aabb: bounding_volume::AABB) -> DebugEntity {
        DebugEntity::new(DebugEntityType::Parry(ParryDebugEntityType::AABB { aabb }))
    }
}
