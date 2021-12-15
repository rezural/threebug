// static EXISTS: AtomicBool = AtomicBool::new(false);
// static GLOBAL_INIT: AtomicUsize = AtomicUsize::new(UNINITIALIZED);

// #[cfg(feature = "std")]
// static SCOPED_COUNT: AtomicUsize = AtomicUsize::new(0);

// const UNINITIALIZED: usize = 0;
// const INITIALIZING: usize = 1;
// const INITIALIZED: usize = 2;

pub fn init() {}

pub mod debug {
    use parry3d::bounding_volume::AABB;
    use threebug_core::ipc::parry;

    pub fn aabb(aabb: impl Into<AABB>) {
        let _debug_msg = parry::ParryDebugEntityType::new_aabb_entity(aabb.into());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
