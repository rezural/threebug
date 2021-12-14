use std::collections::HashMap;

use bevy::prelude::Component;
use bevy_spicy_networking::ConnectionId;

use super::history::History;

#[derive(Component)]
pub struct DebugSession {
    pub conn_id: ConnectionId,
    pub history: History,
}

impl DebugSession {
    pub fn new(conn_id: ConnectionId) -> Self {
        Self {
            conn_id,
            history: History::default(),
        }
    }
}

pub struct DebugSessions {
    sessions: HashMap<ConnectionId, DebugSession>,
}

impl DebugSessions {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn insert(&mut self, session: DebugSession) -> Option<DebugSession> {
        self.sessions.insert(session.conn_id, session)
    }

    pub fn get_mut(&mut self, conn_id: &ConnectionId) -> Option<&mut DebugSession> {
        self.sessions.get_mut(conn_id)
    }

    pub fn first_mut(&mut self) -> Option<&mut DebugSession> {
        let key = self.sessions.keys().last().cloned();
        if let Some(k) = key {
            if let Some(session) = self.sessions.get_mut(&k) {
                return Some(session);
            }
        }
        None
    }
}
