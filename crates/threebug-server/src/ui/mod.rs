use bevy::{prelude::*, window::Windows};
use bevy_egui::EguiContext;

pub mod session;
pub mod sessions;

use crate::resource::session::Sessions;

use self::sessions::SessionsState;

pub fn ui(
    mut ctx: ResMut<EguiContext>,
    mut sessions: ResMut<Sessions>,
    mut sessions_state: ResMut<SessionsState>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    debug_sessions::sessions(&mut *ctx, &mut *sessions, &mut *sessions_state, window);
}

pub mod debug_sessions {
    use bevy::window::Window;
    use bevy_egui::{
        egui::{self, Align2, ComboBox, Grid, ScrollArea, Ui, Vec2},
        EguiContext,
    };

    use crate::resource::session::{Session, Sessions};

    use super::{session::SessionState, sessions::SessionsState};

    pub fn sessions(
        ctx: &mut EguiContext,
        sessions: &mut Sessions,
        sessions_state: &mut SessionsState,
        window: &Window,
    ) {
        egui::Window::new("Sessions")
            .anchor(Align2::LEFT_TOP, Vec2::new(0., 0.))
            .show(ctx.ctx(), |ui| {
                let session_ids = sessions.session_ids();
                if let Some(current) = sessions.current_session_id_mut() {
                    ComboBox::from_id_source("Choose Session")
                        .selected_text(current.clone())
                        .show_ui(ui, |ui| {
                            // info!("current_session: {}", current);
                            for session in session_ids {
                                // info!("session: {}", session);
                                ui.selectable_value(current, session.clone(), session);
                            }
                        });
                    ui.separator();
                    let session_state = sessions_state.current_session_state_mut(sessions).unwrap();
                    session_details(
                        ui,
                        sessions.current_session_mut().unwrap(),
                        session_state,
                        window,
                    );
                }
            });
    }

    pub fn session_details(
        ui: &mut Ui,
        session: &mut Session,
        session_state: &mut SessionState,
        _window: &Window,
    ) {
        // let title = format!("Session: {}", session.name());
        // ui.label(title);
        // let height = window.height() * 0.95;
        ScrollArea::vertical()
            // .max_height(height)
            .show(ui, |ui| {
                Grid::new("Sessions").show(ui, |ui| {
                    for (state, entity) in session_state
                        .state
                        .iter_mut()
                        .zip(session.history.history.iter())
                    {
                        let label = format!("{}", entity);
                        ui.label(label);
                        ui.checkbox(&mut state.visible, "visible");
                        // ui.selectable_value(current, session.clone(), session);
                        ui.end_row();
                    }
                    // for entity in session.history.entities() {}
                });
            });
    }
}

pub mod debug_sessions_multiwindow {
    use bevy::window::Window;
    use bevy_egui::{
        egui::{self, Grid, ScrollArea},
        EguiContext,
    };

    use crate::resource::session::{Session, Sessions};

    pub fn sessions(ctx: &mut EguiContext, sessions: &mut Sessions, window: &Window) {
        egui::Window::new("Sessions").show(ctx.ctx(), |ui| {
            let session_ids = sessions.session_ids();
            if let Some(current) = sessions.current_session_id_mut() {
                let height = window.height() * 0.95;
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

    pub fn session_details(ctx: &mut EguiContext, session: &mut Session, window: &Window) {
        let title = format!("Session: {}", session.name());
        egui::Window::new(title).show(ctx.ctx(), |ui| {
            let height = window.height() * 0.95;
            ScrollArea::vertical().max_height(height).show(ui, |ui| {
                Grid::new("Sessions").show(ui, |ui| {
                    for entity in session.history.entities() {
                        let label = format!("{}", entity);
                        ui.label(label);
                        // ui.selectable_value(current, session.clone(), session);
                        ui.end_row();
                    }
                });
            });
        });
    }
}
