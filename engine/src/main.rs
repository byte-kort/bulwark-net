mod network;
mod json_models;
mod transport;

use crate::network::{
    arp,
    models::NetworkState,
    interface::NetFace,
};
use crate::transport::sender;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = NetworkState::new();
    let iface = NetFace::get_my_iface();

    let targets_ips = arp::generate_ip(&iface);

    let discovery_devices = arp::arp_discovery(&iface, targets_ips)?;

    for device in discovery_devices {
        println!("IP: {}\nMAC: {}\n", device.ip, device.mac);
        state.add_device(device);
    }
    
    for device in &state.devices {
        let event = json_models::DeviceDetected {
            event: "detected_device",
            ip: device.ip.to_string(),
            mac: device.mac.to_string(),
        };

        let json = serde_json::to_string(&event)?;
        sender::send_json(json).await?;
    }

    Ok(())
}
