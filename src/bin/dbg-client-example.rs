use std::net::SocketAddr;

use bevy::prelude::info;
use bevy_spicy_networking::{NetworkClient, NetworkSettings};

fn main() {
    let mut client = NetworkClient::new();
    let ip_address = "127.0.0.1".parse().unwrap();

    info!("Address of the server: {}", ip_address);

    let socket_address = SocketAddr::new(ip_address, 9999);

    client.connect(
        socket_address,
        NetworkSettings {
            max_packet_length: 10 * 1024 * 1024,
        },
    );

    println!("connected");

    client.disconnect();
}
