use threebug_core::Entity;

use crate::resource::session::Session;

// pub enum ViewMode {
//     Normal,
//     Cull,
// }

#[derive(Default)]
pub struct SessionState {
    state: Vec<SessionItemState>,
    selected: Vec<usize>,
}

impl SessionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sync(&mut self, session: &Session) {
        self.state = session
            .history
            .entities()
            .map(|e| SessionItemState {
                entity: e.into(),
                visible: true,
            })
            .collect()
    }
}

pub struct SessionItemState {
    pub entity: Entity,
    pub visible: bool,
}
