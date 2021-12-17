use bevy::prelude::*;

use crate::{render::Spawnable, resource::session::Sessions};

#[derive(Default)]
pub struct SessionsState {
    current_session_id: Option<String>,
}

impl SessionsState {
    /// Create a new, default RenderSessionState
    pub fn new() -> Self {
        Self::default()
    }

    /// Is this session the session we want to be rendering?
    pub fn is_current(&self, sessions: &Sessions) -> bool {
        sessions.current_session_id() == self.current_session_id
    }

    pub fn has_session(&self) -> bool {
        self.current_session_id.is_some()
    }

    /// Set the current session
    pub fn update_current_session(&mut self, sessions: &Sessions) {
        if let Some(current_session) = sessions.current_session() {
            self.current_session_id = Some(current_session.id());
        } else {
            self.current_session_id = None;
        }
    }

    pub fn spawn_current_session(
        &self,
        sessions: &mut Sessions,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        if let Some(current_session_id) = &self.current_session_id {
            debug!("Spawning session: {}", current_session_id);
            if let Some(session) = sessions.get_mut(current_session_id) {
                session.spawn(commands, meshes, materials)
            }
        }
    }

    pub fn despawn_current_session(
        &self,
        sessions: &mut Sessions,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        if let Some(current_session_id) = &self.current_session_id {
            debug!("Despawning session: {}", current_session_id);
            if let Some(session) = sessions.get_mut(current_session_id) {
                session.despawn(commands, meshes, materials)
            }
        }
    }
}
