use std::collections::HashMap;
use bluer::{Adapter, AdapterEvent, Address, AddressType, Device, Result, Session};

pub struct Manager {
    pub devices_connected: HashMap<[u8; 6], String>, 
    session: Session,
    adapter: Adapter,
}

impl Manager {
    pub async fn new() -> Manager {
        let session = Session::new().await.unwrap();
        let adapter = session.default_adapter().await.unwrap();
        adapter.set_powered(true).await;

        Manager { 
            devices_connected: HashMap::new(), 
            session: session.clone(), 
            adapter: adapter.clone(),
        }
    }

    pub async fn start_scan(&self) {
        self.adapter.discover_devices().await;
    }

    pub async fn get_devices(&self) {
        let device_addresses_return = self.adapter.device_addresses().await;
        let device_addresses = match device_addresses_return {
            Ok(device_addresses) => device_addresses,
            Err(error) => panic!("Qdresse found : {:?}", error),
        };

        let mut target_device : Option<Device> = None;
        for device_addresse in device_addresses {
            println!("{:?}", device_addresse);

            let device = self.adapter.device(device_addresse).unwrap();
            let name = device.name().await.unwrap().unwrap();

            println!("Device found: {device_addresse} {name}");
        }
    }
}