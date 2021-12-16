use bevy::prelude::{info, Assets, Commands, Mesh, StandardMaterial};
// use bevy::prelude::Component;
use bevy_spicy_networking::ConnectionId;
use indexmap::IndexMap;

use crate::render::Spawnable;

use super::history::History;

#[derive(Default)]
pub struct SessionRenderState {
    current_session_id: Option<String>,
}

impl SessionRenderState {
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
}

// #[derive(Component)]
pub struct Session {
    pub conn_id: ConnectionId,
    pub history: History,
}

impl Session {
    pub fn new(conn_id: ConnectionId) -> Self {
        Self {
            conn_id,
            history: History::default(),
        }
    }

    pub fn id(&self) -> String {
        self.conn_id.uuid().to_string()
    }
}

impl Spawnable for Session {
    fn spawn(
        &mut self,
        commands: &mut bevy::prelude::Commands,
        meshes: &mut bevy::prelude::Assets<bevy::prelude::Mesh>,
        materials: &mut bevy::prelude::Assets<bevy::prelude::StandardMaterial>,
    ) {
        for debug_entity in self.history.entities_mut() {
            debug_entity.spawn(commands, meshes, materials);
        }
    }

    fn despawn(
        &mut self,
        commands: &mut bevy::prelude::Commands,
        meshes: &mut bevy::prelude::Assets<bevy::prelude::Mesh>,
        materials: &mut bevy::prelude::Assets<bevy::prelude::StandardMaterial>,
    ) {
        for debug_entity in self.history.entities_mut() {
            debug_entity.despawn(commands, meshes, materials);
        }
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.conn_id == other.conn_id
    }
}

pub struct Sessions {
    sessions: IndexMap<String, Session>,
    current_session_id: Option<String>,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            sessions: IndexMap::new(),
            current_session_id: None,
        }
    }

    pub fn insert(&mut self, session: Session) -> Option<Session> {
        // set the current_session to session if it is the first
        if self.sessions.is_empty() {
            self.set_current_session(&session);
        }
        self.sessions.insert(session.id(), session)
    }

    pub fn get_mut(&mut self, session_id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(session_id)
    }

    pub fn set_current_session(&mut self, session: &Session) {
        self.current_session_id = Some(session.id());
    }

    pub fn current_session_id(&self) -> Option<String> {
        self.current_session_id.clone()
    }
    pub fn current_session_id_mut(&mut self) -> Option<&mut String> {
        self.current_session_id.as_mut()
    }

    pub fn current_session(&self) -> Option<&Session> {
        self.current_session_id
            .as_ref()
            .and_then(|conn_id| self.sessions.get(conn_id))
    }

    pub fn current_session_mut(&mut self) -> Option<&mut Session> {
        self.current_session_id
            .as_ref()
            .and_then(|conn_id| self.sessions.get_mut(conn_id))
    }

    pub fn session_ids(&self) -> Vec<String> {
        self.sessions().map(|s| s.id()).collect()
    }

    pub fn sessions(&self) -> impl Iterator<Item = &Session> {
        self.sessions.iter().map(|(_, v)| v)
    }
}

impl Default for Sessions {
    fn default() -> Self {
        Self::new()
    }
}
