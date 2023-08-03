use crate::framework_bluetooth::{Device, DeviceName, DeviceAddress};

use serde::Serialize;
use std::collections::HashMap;
use bluer::{Adapter, Session};
use futures::pin_mut;
use tokio::time;

#[derive(Clone, Serialize)]
pub struct Devices {
    pub device: Vec<Device>,
}

#[derive(Clone)]
pub struct Manager {
    pub devices_connected: HashMap<[u8; 6], String>, 
    session: Session,
    adapter: Adapter,
}

impl Manager {
    pub async fn new() -> Self {
        let session = match Session::new().await {
            Ok(session) => session,
            Err(e) => panic!("New session error : {:?}", e),
        };
        let adapter = match session.default_adapter().await {
            Ok(adapter) => adapter,
            Err(e) => panic!("New adapter error : {:?}", e),
        };
        
        let _ = adapter.set_powered(true).await;

        Self { 
            devices_connected: HashMap::new(), 
            session: session.clone(), 
            adapter: adapter.clone(),
        }
    }

    pub async fn start_scan(&self) {
        let is_discovering = match self.adapter.is_discovering().await {
            Ok(is_discovering) => is_discovering,
            Err(_) => false,
        };

        if is_discovering {
            return;
        }

        let cl = self.adapter.clone();
        tokio::spawn(async move {
            let discover = cl.discover_devices().await.unwrap();
            pin_mut!(discover);

            let timeout = cl.discoverable_timeout().await.unwrap() as u64;
            let scan_duration: time::Duration = time::Duration::from_secs(timeout);
            time::sleep(scan_duration).await;
        });
        
        println!("Stopping discovery");
    }

    pub async fn get_devices(&self) -> Option<Devices> {
        let device_addresses_return = self.adapter.device_addresses().await;
        let device_addresses = match device_addresses_return {
            Ok(device_addresses) => device_addresses,
            Err(error) => panic!("Adresse not found : {:?}", error),
        };

        let mut devices = Devices {
            device: Vec::new(),
        };

        for device_addresse in device_addresses {

            let option_device = self.adapter.device(device_addresse);
            let device = match option_device {
                Ok(d) => d,
                Err(_) => continue,
            };

            let option_name = match device.name().await {
                Ok(n) => n,
                Err(_) => None,
            };

            let name = match option_name {
                Some(n) => n,
                None => String::from("NA"),
            };

            devices.device.push(Device {
                name: DeviceName(name),
                address: DeviceAddress(device.address().to_string()),
            });
        }

        Some(devices)
    }

    pub async fn get_device(&self, address: DeviceAddress) -> Option<Device> {
        let devices = self.get_devices().await.unwrap();
        let device = devices.device.into_iter().find(|d| d.address == address.clone());

        device
    }
}