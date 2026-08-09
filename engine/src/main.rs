mod network;
mod json_models;

use std::io::Write;
use std::os::unix::net::UnixListener;

use crate::network::arp::{self, generate_ip};
use crate::network::models::NetworkState;
use crate::network::interface::NetFace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/tmp/bulwark.sock";

    let _ = std::fs::remove_file(path);

    let listener = UnixListener::bind(path)?;
    
    let (mut stream, _) = listener.accept()?;
    println!("[ENGINE] Server connected");

    let mut state = NetworkState::new();
    let iface = NetFace::get_my_iface();

    let targets_ips = generate_ip(&iface);

    println!("[ENGINE] Starting ARP discovery");
    let discovery_devices = arp::arp_discovery(&iface, targets_ips)?;

    for device in discovery_devices {
        state.add_device(device);
    }


    
    for device in &state.devices {
        let event = json_models::DeviceDetected {
            event: "detected_device",
            ip: device.ip.to_string(),
            mac: device.mac.to_string(),
        };
        let json = serde_json::to_string(&event)?;
        stream.write_all(json.as_bytes())?;
        stream.write_all(b"\n")?;
    }

    Ok(())
}
