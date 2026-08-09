use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

use pnet::datalink::Config;
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::Packet;
use pnet::packet::arp::{ArpOperations, ArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};

use crate::network::models::Device;
use crate::network::interface::NetFace;
use crate::network::frame;

const DISCOVERY_TIMEOUT: Duration = Duration::from_secs(5);
const READ_TIMEOUT: Duration = Duration::from_millis(100);

pub fn arp_discovery(
        iface: &NetFace,
        targets: Vec<Ipv4Addr>,
        ) -> Result<Vec<Device>, Box<dyn std::error::Error>> {

    let config = Config {
        read_timeout: Some(READ_TIMEOUT),
        ..Config::default()
    };
    
    let (mut tx, mut rx) = match datalink::channel(&iface.interface, config) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("failed to create a channel: {}", e),
    };

    for target in targets {
        let mut buf = [0u8; 42];
        frame::create_ethernet_frame(&mut buf, iface.mac, iface.ip, target);
        let _ = tx.send_to(&buf, None).expect("Error with sending frame");
    }

    let mut devices = Vec::new();

    let started_at = Instant::now();

    while started_at.elapsed() <= DISCOVERY_TIMEOUT {
        match rx.next() {
            Ok(packet) => {
                if let Some(value) = parse_arp_reply(packet) {
                    devices.push(value);
                } else {
                    continue
                }
            }
            Err(_) => {
                // Обычно значит что таймаут прошел
            }
        };
    };
    Ok(devices)
}

pub fn parse_arp_reply(packet: &[u8]) -> Option<Device> {
    let ethernet_packet = EthernetPacket::new(packet).unwrap();
    let arp = ArpPacket::new(ethernet_packet.payload()).unwrap();

    if arp.get_protocol_type() != EtherTypes::Ipv4 {
        return None;
    }

    if arp.get_operation() != ArpOperations::Reply {
        return None;
    }

    let sender_ip = arp.get_sender_proto_addr();
    let sender_mac = arp.get_sender_hw_addr();

    let device = Device::new(sender_ip, sender_mac);
    Some(device)
}

pub fn generate_ip(iface: &NetFace) -> Vec<Ipv4Addr> {
    let [a, b, c, _] = iface.ip.octets();
    let prefix = iface.prefix;

    let mut targets = Vec::new();
    
    if prefix == 24u8 {
        for num in 0..255 {
            let target = Ipv4Addr::new(a, b, c, num);
            targets.push(target);
        }
    }
    // Добавить поддержку других префиксов
    targets
}
