use std::net::Ipv4Addr;
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::util::MacAddr;

const ETHERNET_HEADER_LEN: usize = 14;
const ARP_PACKET_LEN: usize = 28;
const ETHERNET_FRAME_LEN: usize =
    ETHERNET_HEADER_LEN + ARP_PACKET_LEN;

pub fn fill_ethernet_header(buf: &mut [u8], source_mac: MacAddr) {
    let mut eth_frame = 
        MutableEthernetPacket::new(buf)
        .unwrap();

    eth_frame.set_source(source_mac);
    eth_frame.set_destination(MacAddr::broadcast());
    eth_frame.set_ethertype(EtherTypes::Arp);
}

pub fn arp_payload(buf: &mut [u8], source_mac: MacAddr, source_ip: Ipv4Addr, target_ip: Ipv4Addr) {
    let mut arp = 
        MutableArpPacket::new(buf)
        .unwrap();

    arp.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp.set_protocol_type(EtherTypes::Ipv4);

    arp.set_hw_addr_len(6);
    arp.set_proto_addr_len(4);

    arp.set_operation(ArpOperations::Request);

    arp.set_sender_hw_addr(source_mac);
    arp.set_sender_proto_addr(source_ip);

    arp.set_target_hw_addr(MacAddr::zero());
    arp.set_target_proto_addr(target_ip);
}

pub fn create_ethernet_frame(buf: &mut [u8],
        source_mac: MacAddr,
        source_ip: Ipv4Addr,
        target_ip: Ipv4Addr,
    ) {
    fill_ethernet_header(&mut buf[..ETHERNET_HEADER_LEN], source_mac);
    arp_payload(&mut buf[ETHERNET_HEADER_LEN..ETHERNET_FRAME_LEN], source_mac, source_ip, target_ip);
}
