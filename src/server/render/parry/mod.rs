use bevy::prelude::*;

use super::{MeshProvider, Spawnable};
use crate::ipc::parry::*;

impl Spawnable for AABB {
    fn spawn(&mut self, commands: &mut bevy::prelude::Commands, meshes: &mut Assets<Mesh>) {
        let mesh = meshes.add(self.mesh());
        let entity = commands
            .spawn_bundle(PbrBundle {
                mesh,
                ..Default::default()
            })
            .id();
        self.entity = Some(entity);
    }

    fn despawn(&mut self, commands: &mut bevy::prelude::Commands, meshes: &mut Assets<Mesh>) {
        if let Some(entity) = self.entity {
            commands.entity(entity).despawn_recursive();
        }
    }
}
