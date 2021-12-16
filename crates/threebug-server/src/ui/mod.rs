use bevy::prelude::ResMut;
use bevy_egui::EguiContext;

use crate::resource::session::Sessions;

pub fn ui(mut ctx: ResMut<EguiContext>, mut sessions: ResMut<Sessions>) {
    debug_sessions::sessions(&mut *ctx, &mut *sessions);
}

pub mod debug_sessions {
    use bevy::prelude::info;
    // use bevy::prelude::info;
    use bevy_egui::{
        egui::{self, ComboBox},
        EguiContext,
    };

    use crate::resource::session::Sessions;

    pub fn sessions(ctx: &mut EguiContext, sessions: &mut Sessions) {
        egui::Window::new("Sessions").show(ctx.ctx(), |ui| {
            let session_ids = sessions.session_ids();
            if let Some(current) = sessions.current_session_id_mut() {
                ComboBox::from_label("Choose Session")
                    .selected_text(current.clone())
                    .show_ui(ui, |ui| {
                        info!("current_session: {}", current);
                        for session in session_ids {
                            info!("session: {}", session);
                            ui.selectable_value(current, session.clone(), session);
                            // ui.selectable_label(current, session.clone(), session);
                        }
                    });
            }
        });
    }
}
