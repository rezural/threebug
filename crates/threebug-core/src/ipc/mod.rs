pub mod parry;

use std::fmt::Display;

use bevy_spicy_networking::*;
use chrono::{DateTime, Local};
use serde::*;

use crate::Entity;

use self::parry::ParryDebugEntityType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DebugEntityType {
    Parry(ParryDebugEntityType),
}

impl Display for DebugEntityType {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebugEntityType::Parry(ptype) => {
                write!(f, "{}", ptype)
            }
        }
    }
}

impl From<DebugEntityType> for Entity {
    fn from(entity_type: DebugEntityType) -> Self {
        match entity_type {
            DebugEntityType::Parry(entity) => entity.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DebugEntity {
    pub timestamp: DateTime<Local>,
    pub entity_type: DebugEntityType,
}

impl From<DebugEntity> for Entity {
    fn from(debug_entity: DebugEntity) -> Self {
        debug_entity.entity_type.into()
    }
}

impl From<&DebugEntity> for Entity {
    fn from(debug_entity: &DebugEntity) -> Self {
        debug_entity.entity_type.clone().into()
    }
}

impl Display for DebugEntity {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.entity_type, self.timestamp)
    }
}

impl DebugEntity {
    pub fn new(entity_type: DebugEntityType) -> Self {
        let timestamp = Local::now();
        Self {
            timestamp,
            entity_type,
        }
    }
}

#[typetag::serde]
impl NetworkMessage for DebugEntity {}

impl ServerMessage for DebugEntity {
    const NAME: &'static str = "bevy_debug::ipc::DebugEntity";
}
