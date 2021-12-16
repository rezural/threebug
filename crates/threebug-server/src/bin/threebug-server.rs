use std::{net::SocketAddr, ops::Deref};

use bevy::{
    diagnostic::LogDiagnosticsPlugin,
    input::mouse::MouseWheel,
    prelude::*,
    render::wireframe::WireframePlugin,
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions},
};
// {
//     core::Time,
//     diagnostic::LogDiagnosticsPlugin,
//     ecs::prelude::*,
//     input::Input,
//     math::Vec3,
//     pbr2::StandardMaterial,
//     prelude::{error, info, App, Assets, KeyCode, Transform},
//     render2::{camera::PerspectiveCameraBundle, mesh::Mesh},
//     PipelinedDefaultPlugins,
// };

use bevy_egui::EguiPlugin;
use bevy_spicy_networking::*;
use smooth_bevy_cameras::{
    controllers::fps_3d::{Fps3dCameraBundle, Fps3dCameraController, Fps3dCameraPlugin},
    LookTransformPlugin,
};

use threebug_core::ipc::DebugEntity;
use threebug_server::resource::session::{Session, SessionRenderState, Sessions};

use threebug_server::ui;

fn main() {
    let mut app = App::build();

    app
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        // .insert_resource(Msaa { samples: 4 })
        .insert_resource(WgpuOptions {
            features: WgpuFeatures {
                // The Wireframe requires NonFillPolygonMode feature
                features: vec![WgpuFeature::NonFillPolygonMode],
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        // bevy spicy networking
        .add_plugin(bevy_spicy_networking::ServerPlugin)
        // smooth bevy cameras
        .add_plugin(LookTransformPlugin)
        .add_plugin(Fps3dCameraPlugin::default())
        // bevy egui
        .add_plugin(EguiPlugin)
        .add_system(ui::ui.system())
        .add_startup_system(setup.system())
        .add_system(fps.system())
        .add_system(cursor_grab_system.system())
        .add_system(render.system());

    // Register parry server messages
    register_server_network_messages(&mut app);
    app.add_startup_system(setup_networking.system())
        .add_system(handle_connection_events.system())
        .add_system(handle_messages.system());

    app.insert_resource(Sessions::new());
    app.insert_resource(SessionRenderState::new());

    app.run();
}

fn setup_networking(mut net: ResMut<NetworkServer>) {
    let ip_address = "127.0.0.1".parse().expect("Could not parse ip address");

    let socket_address = SocketAddr::new(ip_address, 9999);

    info!("Address of the server: {}", socket_address);

    match net.listen(socket_address) {
        Ok(_) => (),
        Err(err) => {
            error!("Could not start listening: {}", err);
            panic!();
        }
    }

    info!("Started listening for new connections!");
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(Fps3dCameraBundle::new(
        Fps3dCameraController {
            enabled: false,
            ..Default::default()
        },
        PerspectiveCameraBundle::default(),
        Vec3::new(0.0, 0.0, 15.0),
        Vec3::new(0., 0., 0.),
    ));
}

fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sessions: ResMut<Sessions>,
    mut session_render_state: ResMut<SessionRenderState>,
) {
    if session_render_state.is_current(&*sessions) {
        // FIXME: check if we need to update
        // if session.history.is_dirty() {
        //     info!("session dirty");
        //     for v in session.history.dirty_entities() {
        //         info!("spawning entity");

        //         match &mut v.entity_type {
        //             ipc::DebugEntityType::Parry(ptype) => match ptype {
        //                 ipc::parry::ParryDebugEntityType::AABB { aabb } => {
        //                     aabb.spawn(&mut commands, &mut *meshes, &mut *materials);
        //                 }
        //             },
        //         }
        //     }
        //     session.history.clean();
        // }
    } else {
        session_render_state.despawn_current_session(
            &mut *sessions,
            &mut commands,
            &mut *meshes,
            &mut *materials,
        );
        session_render_state.update_current_session(&*sessions);

        session_render_state.spawn_current_session(
            &mut *sessions,
            &mut commands,
            &mut *meshes,
            &mut *materials,
        );
    }
}

fn handle_connection_events(
    mut _commands: Commands,
    _net: Res<NetworkServer>,
    mut network_events: EventReader<ServerNetworkEvent>,
    mut sessions: ResMut<Sessions>,
) {
    // info!("handle_connection_events");
    for event in network_events.iter() {
        info!("got event");
        if let ServerNetworkEvent::Connected(conn_id) = event {
            let session = Session::new(*conn_id);
            sessions.insert(session);

            //TODO: send accept accepted to client
            info!("New client connected: {}", conn_id);
        }
    }
}

fn handle_messages(
    mut new_messages: EventReader<NetworkData<threebug_core::ipc::DebugEntity>>,
    // net: Res<NetworkServer>,
    mut sessions: ResMut<Sessions>,
) {
    for message in new_messages.iter() {
        info!(
            "Received debug message from client: {}, {:?}",
            message.timestamp, message.entity_type
        );

        let conn_id = message.source();
        if let Some(session) = sessions.get_mut(&conn_id.uuid().to_string()) {
            let inner = message.deref();
            session.history.push(inner.clone());
            info!("{} entitiees", session.history.len());
        }
    }
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut controllers: Query<&mut Fps3dCameraController>,
) {
    let window = windows.get_primary_mut().unwrap();

    let mut controller = controllers.single_mut().unwrap();
    if btn.just_pressed(MouseButton::Left) {
        info!("enabling fps 3d controller");

        controller.enabled = true;
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        info!("disabling fps 3d controller");
        controller.enabled = false;
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}

fn fps(
    _keyboard: Res<Input<KeyCode>>,
    _mouse: Res<Input<MouseButton>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    _time: Res<Time>,
    mut fps: Query<&mut Fps3dCameraController>,
) {
    if let Ok(mut fps) = fps.single_mut() {
        for event in mouse_wheel_events.iter() {
            //FIXME: move this into some kind of easing function thingy
            let delta = if fps.translate_sensitivity <= 0.2 {
                0.01
            } else if fps.translate_sensitivity <= 1.0 {
                0.1
            } else {
                0.3
            };
            let delta = event.y * delta;
            fps.translate_sensitivity += delta;
            fps.translate_sensitivity = fps.translate_sensitivity.clamp(0.01, 10.0);
            info!(
                "Changing translate sensitivity by {} to {}",
                delta, fps.translate_sensitivity
            );
        }
    }
}

pub fn register_server_network_messages(app: &mut AppBuilder) {
    app.listen_for_server_message::<DebugEntity>();
}
