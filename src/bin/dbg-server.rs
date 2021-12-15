use std::{net::SocketAddr, ops::Deref};

use bevy::{
    diagnostic::LogDiagnosticsPlugin,
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

use bevy_debug::{
    ipc,
    server::{
        render::Spawnable,
        store::{DebugSession, DebugSessions},
    },
};
use bevy_spicy_networking::*;
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

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
        .add_plugin(FpsCameraPlugin::default())
        .add_startup_system(setup.system())
        // .add_system(movement.system())
        .add_system(render.system());

    // Register parry server messages
    ipc::register_server_network_messages(&mut app);
    app.add_startup_system(setup_networking.system())
        .add_system(handle_connection_events.system())
        .add_system(handle_messages.system());

    app.insert_resource(DebugSessions::new());

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
    commands.spawn_bundle(FpsCameraBundle::new(
        FpsCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(0.0, 0.0, 15.0),
        Vec3::new(0., 0., 0.),
    ));
}

fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sessions: ResMut<DebugSessions>,
) {
    //FIXME: allow multiple sessions
    if let Some(session) = sessions.first_mut() {
        if session.history.is_dirty() {
            info!("session dirty");
            for v in session.history.dirty_entities() {
                info!("spawning entity");

                match &mut v.entity_type {
                    ipc::DebugEntityType::Parry(ptype) => match ptype {
                        ipc::parry::ParryDebugEntityType::AABB { aabb } => {
                            aabb.spawn(&mut commands, &mut *meshes, &mut *materials);
                        }
                    },
                }
            }
            session.history.clean();
        }
    }
}

fn handle_connection_events(
    mut _commands: Commands,
    _net: Res<NetworkServer>,
    mut network_events: EventReader<ServerNetworkEvent>,
    mut sessions: ResMut<DebugSessions>,
) {
    // info!("handle_connection_events");
    for event in network_events.iter() {
        info!("got event");
        if let ServerNetworkEvent::Connected(conn_id) = event {
            let session = DebugSession::new(*conn_id);
            sessions.insert(session);

            //TODO: send accept accepted to client
            info!("New client connected: {}", conn_id);
        }
    }
}

fn handle_messages(
    mut new_messages: EventReader<NetworkData<bevy_debug::ipc::DebugEntity>>,
    // net: Res<NetworkServer>,
    mut sessions: ResMut<DebugSessions>,
) {
    for message in new_messages.iter() {
        info!(
            "Received debug message from client: {}, {:?}",
            message.timestamp, message.entity_type
        );

        let conn_id = message.source();
        if let Some(session) = sessions.get_mut(&conn_id) {
            let inner = message.deref();
            session.history.push(inner.clone());
            info!("{} entitiees", session.history.len());
        }
    }
}
// #[derive(Component)]
struct Movable;

fn movement(input: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<&mut Transform>) {}
