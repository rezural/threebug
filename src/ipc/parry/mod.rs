use bevy::prelude::App;
use bevy_spicy_networking::*;
use chrono::{DateTime, Local};
use parry3d::bounding_volume;
use serde::*;

#[derive(Serialize, Deserialize)]
pub struct AABB {
    pub timestamp: DateTime<Local>,
    pub aabb: bounding_volume::AABB,
}

impl AABB {
    pub fn new(aabb: bounding_volume::AABB) -> Self {
        Self {
            timestamp: Local::now(),
            aabb,
        }
    }
}

#[typetag::serde]
impl NetworkMessage for AABB {}

impl ServerMessage for AABB {
    const NAME: &'static str = "bevy_debug_server::parry::AABB";
}

pub fn register_server_network_messages(app: &mut App) {
    app.listen_for_server_message::<AABB>();
}
