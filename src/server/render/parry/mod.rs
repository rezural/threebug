use bevy::prelude::*;

use super::{MeshProvider, Spawnable};
use crate::ipc::parry::*;

impl Spawnable for AABB {
    fn spawn(
        &mut self,
        commands: &mut bevy::prelude::Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        let mesh = meshes.add(self.mesh());
        let entity = commands
            .spawn_bundle(PbrBundle {
                mesh,
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                ..Default::default()
            })
            .id();
        self.entity = Some(entity);
    }

    fn despawn(
        &mut self,
        commands: &mut bevy::prelude::Commands,
        _meshes: &mut Assets<Mesh>,
        _materials: &mut Assets<StandardMaterial>,
    ) {
        if let Some(entity) = self.entity {
            commands.entity(entity).despawn_recursive();
        }
    }
}
