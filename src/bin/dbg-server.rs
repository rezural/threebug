use std::net::SocketAddr;

use bevy::{
    core::Time,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::prelude::*,
    input::Input,
    math::{Quat, Vec3},
    pbr2::{
        AmbientLight, DirectionalLight, DirectionalLightBundle, PbrBundle, PointLight,
        PointLightBundle, StandardMaterial,
    },
    prelude::{error, info, App, Assets, BuildChildren, KeyCode, Transform},
    render2::{
        camera::{OrthographicProjection, PerspectiveCameraBundle},
        color::Color,
        mesh::{shape, Mesh},
    },
    PipelinedDefaultPlugins,
};

use bevy_debug::ipc;
use bevy_spicy_networking::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(PipelinedDefaultPlugins)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(movement)
        .add_plugin(bevy_spicy_networking::ServerPlugin);

    // Register parry server messages
    ipc::parry::register_server_network_messages(&mut app);
    app.add_startup_system(setup_networking)
        .add_system(handle_connection_events);

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

#[derive(Component)]
struct DebugClient(ConnectionId);

fn handle_connection_events(
    mut commands: Commands,
    net: Res<NetworkServer>,
    mut network_events: EventReader<ServerNetworkEvent>,
) {
    // info!("handle_connection_events");
    for event in network_events.iter() {
        info!("got event");
        if let ServerNetworkEvent::Connected(conn_id) = event {
            commands.spawn_bundle((DebugClient(*conn_id),));

            // Broadcasting sends the message to all connected players! (Including the just connected one in this case)
            // net.broadcast(shared::NewChatMessage {
            //     name: String::from("SERVER"),
            //     message: format!("New user connected; {}", conn_id),
            // });
            info!("New client connected: {}", conn_id);
        }
    }
}

fn handle_aabb_messages(
    mut new_messages: EventReader<NetworkData<bevy_debug::ipc::parry::AABB>>,
    net: Res<NetworkServer>,
) {
    for message in new_messages.iter() {
        let user = message.source();

        info!("Received aabb from user: {:?}", message.aabb);
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

#[derive(Component)]
struct Movable;

fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }

        transform.translation += time.delta_seconds() * 2.0 * direction;
    }
}
