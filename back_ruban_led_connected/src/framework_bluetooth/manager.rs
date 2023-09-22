use crate::framework_bluetooth::{Device, DeviceAddress};

use bluer::agent::{Agent, ReqResult, RequestPasskey, RequestPinCode};
use bluer::{Adapter, Session};

use futures::pin_mut;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use tokio::time;

#[derive(Serialize)]
pub struct Devices {
    pub device: Vec<Device>,
}

#[derive(Clone)]
pub struct Manager {
    pub devices_connected: HashMap<DeviceAddress, Device>,
    session: Session,
    adapter: Adapter,
}

impl Manager {
    pub async fn new() -> Self {
        let session = match Session::new().await {
            Ok(session) => session,
            Err(e) => panic!("New session error : {:?}", e),
        };

        let agent = Agent {
            request_pin_code: Some(Box::new(|req| Box::pin(Self::request_pin_code(req)))),
            request_passkey: Some(Box::new(|req| Box::pin(Self::request_passkey(req)))),
            ..Default::default()
        };
        let _handle_agent = session.register_agent(agent).await;

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

    async fn request_pin_code(_req: RequestPinCode) -> ReqResult<String> {
        println!("{:?}", _req);
        println!("call pin");
        Ok("1234".into())
    }

    async fn request_passkey(_req: RequestPasskey) -> ReqResult<u32> {
        println!("{:?}", _req);
        println!("passkey");
        Ok(1234)
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
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                let discover = cl.discover_devices().await.unwrap();
                pin_mut!(discover);

                let timeout = cl.discoverable_timeout().await.unwrap() as u64;
                let scan_duration: time::Duration = time::Duration::from_secs(timeout);
                time::sleep(scan_duration).await;
            });
        });
    }

    pub async fn get_devices(&self) -> Option<Devices> {
        let device_addresses_return = self.adapter.device_addresses().await;
        let device_addresses = match device_addresses_return {
            Ok(device_addresses) => device_addresses,
            Err(error) => panic!("Adresse not found : {:?}", error),
        };

        let mut devices = Devices { device: Vec::new() };

        for device_addresse in device_addresses {
            let option_device = self.adapter.device(device_addresse);
            let device = match option_device {
                Ok(d) => d,
                Err(_) => continue,
            };

            devices.device.push(Device::from(device.clone()).await);
        }

        Some(devices)
    }

    pub async fn get_device(&self, address: DeviceAddress) -> Option<Device> {
        let devices = self.get_devices().await.unwrap();
        let device = devices
            .device
            .into_iter()
            .find(|d| d.address == address.clone());

        device
    }

    pub async fn connect_device(
        &mut self,
        address: DeviceAddress,
    ) -> Result<Receiver<String>, &'static str> {
        let mut device = self.get_device(address.clone()).await.unwrap();
        device.pair().await;
        let reponse = device.connect().await;
        self.devices_connected
            .insert(address.clone(), device.clone());

        reponse
    }

    pub async fn send_device(
        &self,
        address: DeviceAddress,
        message: String
    ) {
        match self.devices_connected.get(&address) {
            Some(device) => device.send(message),
            None => ()
        }
    }

}
