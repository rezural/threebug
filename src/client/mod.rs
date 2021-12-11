pub mod debug {
    use parry3d::bounding_volume::AABB;

    use crate::ipc;

    pub fn aabb(aabb: impl Into<AABB>) {
        let _debug_msg = ipc::parry::ParryDebugEntityType::new_aabb_entity(aabb.into());
    }
}
