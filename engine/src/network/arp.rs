use std::net::Ipv4Addr;

use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::Packet;
use pnet::packet::arp::{ArpOperations, ArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};

use crate::network::models::Device;
use crate::network::interface::NetFace;
use crate::network::frame;

pub fn arp_discovery(
        iface: &NetFace,
        target_ip: Ipv4Addr,
        ) -> Result<Device, Box<dyn std::error::Error>> {

    let (mut tx, mut rx) = match datalink::channel(&iface.interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("failed to create a channel: {}", e),
    };

    let mut buf = [0u8; 42];
    frame::create_ethernet_frame(&mut buf, iface.mac, iface.ip, target_ip);

    tx.send_to(&buf, None).expect("Error with sending frame");
    println!("[+] Frame sended");

    let device = loop {
        match rx.next() {
            Ok(packet) => {
                println!("[+] Packet recieved\n");
                if let Some(value) = parse_arp_reply(packet, target_ip) {
                    break value;
                } else {
                    continue
                }
            }
            Err(e) => {
                eprintln!("[!] Recieve error: {}", e);
            }
        };
    };
    Ok(device)
}

pub fn parse_arp_reply(packet: &[u8], target_ip: Ipv4Addr) -> Option<Device> {
    let ethernet_packet = EthernetPacket::new(packet).unwrap();
    let arp = ArpPacket::new(ethernet_packet.payload()).unwrap();

    if arp.get_protocol_type() != EtherTypes::Ipv4 {
        println!("[#] Packet does not use IPv4 protocol");
        return None;
    }

    if arp.get_operation() != ArpOperations::Reply {
        println!("[#] ARP opration is not a reply");
        return None;
    }

    let sender_ip = arp.get_sender_proto_addr();
    let sender_mac = arp.get_sender_hw_addr();

    if sender_ip != target_ip {
        println!("[#] The sender's IP does not match the target's IP");
        return None;
    }

    println!("[+] This is a required packet!!!!!!!!!!!!!!!");
    let device = Device::new(sender_ip, sender_mac);
    Some(device)
}
