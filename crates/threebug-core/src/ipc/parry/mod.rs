use parry3d::bounding_volume;
use serde::*;

use super::{DebugEntity, DebugEntityType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ParryDebugEntityType {
    AABB { aabb: AABB },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AABB {
    pub entity: Option<u64>,
    pub aabb: bounding_volume::AABB,
}

impl AABB {
    pub fn new(aabb: bounding_volume::AABB) -> Self {
        Self { entity: None, aabb }
    }
}

impl ParryDebugEntityType {
    pub fn new_aabb_entity(aabb: bounding_volume::AABB) -> DebugEntity {
        DebugEntity::new(DebugEntityType::Parry(ParryDebugEntityType::AABB {
            aabb: AABB::new(aabb),
        }))
    }
}
