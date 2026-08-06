use crate::models::device::Device;

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
