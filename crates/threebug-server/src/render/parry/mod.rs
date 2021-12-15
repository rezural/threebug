use bevy::{prelude::*, render::wireframe::Wireframe};

use super::{MeshProvider, Spawnable};
use threebug_core::ipc::parry::*;

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
                material: materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into()),
                ..Default::default()
            })
            .insert(Wireframe)
            .id();
        self.entity = Some(entity.to_bits());
    }

    fn despawn(
        &mut self,
        commands: &mut bevy::prelude::Commands,
        _meshes: &mut Assets<Mesh>,
        _materials: &mut Assets<StandardMaterial>,
    ) {
        if let Some(entity) = self.entity {
            commands
                .entity(Entity::from_bits(entity))
                .despawn_recursive();
        }
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
