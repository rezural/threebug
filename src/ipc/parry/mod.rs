use bevy::prelude::*;
use parry3d::bounding_volume;
use serde::*;

use crate::server::render::MeshProvider;

use super::{DebugEntity, DebugEntityType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ParryDebugEntityType {
    AABB { aabb: AABB },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AABB {
    pub entity: Option<Entity>,
    aabb: bounding_volume::AABB,
}

impl AABB {
    pub fn new(aabb: bounding_volume::AABB) -> Self {
        Self { entity: None, aabb }
    }
}

impl MeshProvider for AABB {
    fn mesh(&self) -> Mesh {
        let mins = self.aabb.mins;
        let maxs = self.aabb.maxs;
        let mins = Vec3::new(mins.x, mins.y, mins.z);
        let maxs = Vec3::new(maxs.x, maxs.y, maxs.z);
        bevy::prelude::shape::Box::from_min_max(mins, maxs).into()
    }
}

impl ParryDebugEntityType {
    pub fn new_aabb_entity(aabb: bounding_volume::AABB) -> DebugEntity {
        DebugEntity::new(DebugEntityType::Parry(ParryDebugEntityType::AABB {
            aabb: AABB::new(aabb),
        }))
    }
}
