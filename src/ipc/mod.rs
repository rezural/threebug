pub mod parry;

use bevy::prelude::App;
use bevy_spicy_networking::*;
use chrono::{DateTime, Local};
use serde::*;

use self::parry::ParryDebugEntityType;

#[derive(Debug, Serialize, Deserialize)]
pub enum DebugEntityType {
    Parry(ParryDebugEntityType),
}

#[derive(Serialize, Deserialize)]
pub struct DebugEntity {
    pub timestamp: DateTime<Local>,
    pub entity_type: DebugEntityType,
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

pub fn register_server_network_messages(app: &mut App) {
    app.listen_for_server_message::<DebugEntity>();
}
