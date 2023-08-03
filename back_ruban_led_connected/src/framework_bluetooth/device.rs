use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize)]
pub struct DeviceName(pub String);

impl PartialEq for DeviceName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeviceAddress(pub String);

impl PartialEq for DeviceAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone, Serialize)]
pub struct Device {
    pub name: DeviceName,
    pub address: DeviceAddress,
}
