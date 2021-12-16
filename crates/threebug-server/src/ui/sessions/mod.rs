use std::collections::HashMap;

use bevy::prelude::*;

use crate::{render::Spawnable, resource::session::Sessions};

use super::session::SessionState;

#[derive(Default)]
pub struct SessionsState {
    current_session_id: Option<String>,
    sessions_state: HashMap<String, SessionState>,
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

    /// Set the current session
    pub fn update_current_session(&mut self, sessions: &Sessions) {
        if let Some(current_session) = sessions.current_session() {
            self.current_session_id = Some(current_session.id());
        } else {
            self.current_session_id = None;
        }
    }

    pub fn sync_session_states(&mut self, sessions: &Sessions) {
        for id in sessions.keys() {
            self.sessions_state
                .entry(id.clone())
                .or_insert_with(SessionState::default);
        }
    }

    pub fn current_session_state_mut(&mut self, sessions: &Sessions) -> Option<&mut SessionState> {
        sessions.current_session_id().map(|id| {
            self.sessions_state
                .entry(id)
                .or_insert_with(SessionState::default)
        })
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut SessionState> {
        self.sessions_state.get_mut(key)
    }

    pub fn spawn_current_session(
        &self,
        sessions: &mut Sessions,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        if let Some(current_session_id) = &self.current_session_id {
            info!("Spawning session: {}", current_session_id);
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
            info!("Despawning session: {}", current_session_id);
            if let Some(session) = sessions.get_mut(current_session_id) {
                session.despawn(commands, meshes, materials)
            }
        }
    }
}
