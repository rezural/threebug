use std::{error::Error, net::SocketAddr};

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

    let aabb = parry3d::bounding_volume::AABB::new(
        Point::new(-1.0, -1.0, -1.0),
        Point::new(1.0, 1.0, 1.0),
    );

    match client.send_message(bevy_debug::ipc::parry::AABB::new(aabb)) {
        Ok(_) => info!("sent aabb"),
        Err(e) => error!("Couldnt send aabb to server: {:?}", e),
    }

    client.disconnect();

    Ok(())
}
