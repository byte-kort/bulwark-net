// Собирает информацию, делает NetworkState::add(state)

use crate::models::device::Device;

pub fn scan() -> Vec<Device> {
    let devices = vec![
        Device {
            ip: "8.8.8.8".into(),
            mac: "AA:BB:CC".into(),
        }
    ];

    devices
}
