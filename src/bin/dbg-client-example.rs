use core::time;
use std::{error::Error, net::SocketAddr, thread};

use bevy::prelude::{error, info};
use bevy_spicy_networking::{NetworkSettings, StandaloneNetworkClient};
use parry3d::math::Point;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let mut client = StandaloneNetworkClient::new();
    let ip_address = "127.0.0.1".parse().unwrap();

    let socket_address = SocketAddr::new(ip_address, 9999);

    info!("Address of the server: {}", socket_address);

    client.connect(
        socket_address,
        NetworkSettings {
            max_packet_length: 10 * 1024 * 1024,
        },
    )?;

    info!("connected");

    for i in 1..10 {
        let i = i as f32;
        let neg = i * -1.0;
        let aabb =
            parry3d::bounding_volume::AABB::new(Point::new(neg, neg, neg), Point::new(i, i, i));

        let debug_entity_type =
            bevy_debug::ipc::parry::ParryDebugEntityType::new_aabb_entity(aabb.into());

        match client.send_message(debug_entity_type) {
            Ok(_) => info!("sent aabb"),
            Err(e) => error!("Couldnt send aabb to server: {:?}", e),
        }
        thread::sleep(time::Duration::from_millis(1000));
    }

    client.disconnect();

    Ok(())
}
