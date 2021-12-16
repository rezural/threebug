use bevy::{prelude::*, window::Windows};
use bevy_egui::EguiContext;

use crate::resource::session::Sessions;

pub fn ui(mut ctx: ResMut<EguiContext>, mut sessions: ResMut<Sessions>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    debug_sessions::sessions(&mut *ctx, &mut *sessions, window);
}

pub mod debug_sessions {
    use bevy::window::Window;
    use bevy_egui::{
        egui::{self, Grid, ScrollArea},
        EguiContext,
    };

    use crate::resource::session::Sessions;

    pub fn sessions(ctx: &mut EguiContext, sessions: &mut Sessions, window: &Window) {
        egui::Window::new("Sessions").show(ctx.ctx(), |ui| {
            let session_ids = sessions.session_ids();
            if let Some(current) = sessions.current_session_id_mut() {
                let height = window.height() * 0.8;
                ScrollArea::vertical().max_height(height).show(ui, |ui| {
                    Grid::new("Sessions").show(ui, |ui| {
                        for session in session_ids {
                            ui.selectable_value(current, session.clone(), session);
                            ui.end_row();
                        }
                    });
                })
            }
        });
    }
}
