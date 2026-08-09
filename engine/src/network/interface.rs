use std::net::Ipv4Addr;

use pnet::datalink;
use pnet::util::MacAddr;

pub struct NetFace {
    pub interface: pnet::datalink::NetworkInterface,
    pub mac: MacAddr,
    pub ip: Ipv4Addr,
    pub prefix: u8,
}

impl NetFace {
    pub fn get_my_iface() -> Self {
        let iface = datalink::interfaces()
            .into_iter()
            .find(|iface| {
                !iface.is_loopback()
                    && iface.mac.is_some()
                    && iface.ips.iter().any(|ip| ip.is_ipv4())
            })
            .expect("Cannot find internet interface");

        let mac = iface.mac.unwrap();

        let ip_network = iface
            .ips
            .iter()
            .find(|ip| ip.is_ipv4())
            .expect("Interface has no IPv4 address");

        let ip = match ip_network.ip() {
            std::net::IpAddr::V4(ip) => ip,
            _ => unreachable!(),
        };

        let prefix = ip_network.prefix();

        Self {
            interface: iface,
            mac,
            ip,
            prefix,
        }
    }
}
