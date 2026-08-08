use std::net::Ipv4Addr;

use pnet::datalink;
use pnet::util::MacAddr;

pub struct NetFace {
    pub interface: pnet::datalink::NetworkInterface,
    pub name: String,
    pub mac: MacAddr,
    pub ip: Ipv4Addr,
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
        let name = iface.name.clone();

        let ip = iface
            .ips
            .iter()
            .find_map(|ip| {
                if let std::net::IpAddr::V4(ip) = ip.ip() {
                    Some(ip)
                } else {
                    None
                }
            })
            .expect("Interface has no IPv4 address");
        Self {
            interface: iface,
            name,
            mac,
            ip,
        }

    }

    pub fn discover(&mut self) {
        todo!();
    }
}
