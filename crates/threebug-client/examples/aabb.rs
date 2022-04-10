use core::time;
use std::{error::Error, net::SocketAddr, thread};

use bevy_spicy_networking::{NetworkSettings, StandaloneNetworkClient};
use parry3d::na::Point3;
use rand::Rng;
use structopt::StructOpt;
use tracing::{error, info};

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short, long, default_value = "10")]
    count: usize,
    #[structopt(short, long, default_value = "10")]
    volume_radius: f32,
    #[structopt(short, long, default_value = "2.0")]
    aabb_radius: f32,
    #[structopt(short, long, default_value = "0.5")]
    wait: f32,
    #[structopt(short, long)]
    random_radius: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let opt = Options::from_args();

    let mut client = StandaloneNetworkClient::new();
    let ip_address = "127.0.0.1".parse().unwrap();

    let socket_address = SocketAddr::new(ip_address, 9876);

    info!("Address of the server: {}", socket_address);

    client.connect(
        socket_address,
        NetworkSettings {
            max_packet_length: 10 * 1024 * 1024,
        },
    )?;

    info!("Connected");

    let mut rng = rand::thread_rng();

    for _ in 0..opt.count {
        let vrange = -opt.volume_radius..opt.volume_radius;
        let centre = Point3::origin().map(|_: f32| rng.gen_range(vrange.clone()));

        let radius = if opt.random_radius {
            opt.aabb_radius
        } else {
            rng.gen::<f32>() * opt.aabb_radius
        };
        let mins = centre.map(|c| c - radius);
        let maxs = centre.map(|c| c + radius);

        let aabb = parry3d::bounding_volume::AABB::new(mins, maxs);

        let debug_entity_type =
            threebug_core::ipc::parry::ParryDebugEntityType::new_aabb_entity(aabb);

        match client.send_message(debug_entity_type) {
            Ok(_) => info!("Sent aabb"),
            Err(e) => error!("Couldnt send aabb to server: {:?}", e),
        }
        thread::sleep(time::Duration::from_secs_f32(opt.wait));
    }

    client.disconnect();

    Ok(())
}
