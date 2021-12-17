use std::fmt::Display;

use parry3d::bounding_volume;
use serde::*;

use crate::Entity;

use super::{DebugEntity, DebugEntityType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ParryDebugEntityType {
    AABB { aabb: AABB },
}

impl ParryDebugEntityType {
    pub fn new_aabb_entity(aabb: bounding_volume::AABB) -> DebugEntity {
        DebugEntity::new(DebugEntityType::Parry(ParryDebugEntityType::AABB {
            aabb: AABB::new(aabb),
        }))
    }
}

impl Display for ParryDebugEntityType {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParryDebugEntityType::AABB { .. } => write!(f, "aabb"),
        }
    }
}

impl From<ParryDebugEntityType> for Entity {
    fn from(entity_type: ParryDebugEntityType) -> Self {
        match entity_type {
            ParryDebugEntityType::AABB { aabb } => aabb.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AABB {
    pub entity: Option<Entity>,
    pub aabb: bounding_volume::AABB,
}

impl AABB {
    pub fn new(aabb: bounding_volume::AABB) -> Self {
        Self { entity: None, aabb }
    }
}

impl From<AABB> for Entity {
    fn from(aabb: AABB) -> Self {
        aabb.entity.unwrap_or_else(Entity::invalid)
    }
}
