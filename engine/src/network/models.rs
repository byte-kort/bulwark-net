use std::net::Ipv4Addr;
use pnet::util::MacAddr;

#[derive(Debug)]
pub struct Device {
    pub ip: Ipv4Addr,
    pub mac: MacAddr,
}

impl Device {
    pub fn new(ip: Ipv4Addr, mac: MacAddr) -> Self {
        Self {
            ip,
            mac,
        }
    }
}

pub struct NetworkState {
    pub devices: Vec<Device>
}

impl NetworkState {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
    }
}

