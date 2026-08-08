mod network;
use std::net::Ipv4Addr;

use crate::network::arp;
use crate::network::models::NetworkState;
use crate::network::interface::NetFace;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Создал let state = NetworkState::new();
    // сделал arp::discovery(&state);
    
    let mut state = NetworkState::new();
    let iface = NetFace::get_my_iface();

    let target_ip = Ipv4Addr::new(8, 8, 8, 8);
    let device = arp::arp_discovery(&iface, &mut state, target_ip)?;

    state.add_device(device);
    println!("[MAIN] Device has been added");

    Ok(())
}
