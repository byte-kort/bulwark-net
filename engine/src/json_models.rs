use serde::Serialize;

#[derive(Serialize)]
pub struct DeviceDetected {
    pub event: &'static str,
    pub ip: String,
    pub mac: String,
}
