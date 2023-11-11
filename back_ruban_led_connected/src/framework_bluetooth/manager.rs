use crate::framework_bluetooth::{Device, DeviceAddress};

use actix_web::cookie::Delta;
use bluer::agent::{Agent, ReqResult, RequestPasskey, RequestPinCode};
use bluer::{Adapter, Session};

use futures::pin_mut;
use serde::{ Serialize, Deserialize };
use std::collections::HashMap;
use tokio::time;
use chrono::{Local, DateTime};
use std::sync::{Arc, Mutex};

mod option_date_serialiser {
    use chrono::{DateTime, Local, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(
        date: &Option<DateTime<Local>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *date {
            Some(ref dt) => {
                let s = format!("{}", &dt.format(FORMAT));
                serializer.serialize_str(&s)
                
            },
            None => serializer.serialize_none(),
        }
        
    }
}

#[derive(Serialize)]
pub struct Scan {
    pub starting: bool,
    #[serde(with = "option_date_serialiser")]
    pub started_time: Option<DateTime<Local>>,
    pub timeout: u64
}

#[derive(Serialize, Debug)]
pub struct Devices {
    pub device: Vec<Device>,
}

#[derive(Deserialize, Debug)]
pub struct DevicesFilters {
    pub connected: bool
}

impl Devices {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Clone)]
pub struct Manager {
    pub devices_connected: HashMap<DeviceAddress, Device>,
    session: Session,
    adapter: Adapter,
    scan_started: Arc<Mutex<Option<DateTime<Local>>>>
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
            scan_started: Arc::new(Mutex::new(None))
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

    pub async fn scan(&self) -> Scan {
        let mut is_discovering = match self.adapter.is_discovering().await {
            Ok(is_discovering) => is_discovering,
            Err(_) => false,
        };
        
        let cl = self.adapter.clone();
        let timeout = cl.discoverable_timeout().await.unwrap() as u64;

        let scan_started = self.scan_started.lock().unwrap();

        match *scan_started {
            Some(sarted_date) => {
                let date_now = Local::now();
                let date_delta = date_now.timestamp() as u64 - sarted_date.timestamp() as u64;

                if date_delta > 180 || !is_discovering {
                    is_discovering = false;
                }
            },
            None => ()
        }

        Scan {
            starting: is_discovering,
            started_time: *scan_started,
            timeout: timeout
        }
    }

    pub async fn start_scan(&mut self) -> Scan {
        let is_discovering = match self.adapter.is_discovering().await {
            Ok(is_discovering) => is_discovering,
            Err(_) => false,
        };

        let mut scan = self.scan().await;

        if is_discovering {
            return scan;
        }

        
        let mut scan_started = self.scan_started.lock().unwrap();
        *scan_started = Some(Local::now());

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

        scan.started_time = *scan_started;
        scan.starting = true;
        scan
    }

    pub async fn get_devices(&self, filters:Option<DevicesFilters>) -> Option<Devices> {
        let device_addresses_return = self.adapter.device_addresses().await;
        let device_addresses = match device_addresses_return {
            Ok(device_addresses) => device_addresses,
            Err(error) => panic!("Adresse not found : {:?}", error),
        };

        let mut devices = Devices { device: Vec::new() };

        for device_addresse in device_addresses {
            let option_device = self.adapter.device(device_addresse);
            let device = match option_device {
                Ok(d) => Device::from(d).await,
                Err(_) => continue,
            };

            match &filters {
                Some(filter) => {
                    if filter.connected && device.connected {
                        devices.device.push(device);
                    }
                }, 
                None => devices.device.push(device)
            }
        }

        Some(devices)
    }

    pub async fn get_device(&self, address: DeviceAddress) -> Option<Device> {
        let devices = self.get_devices(None).await.unwrap();
        let device = devices
            .device
            .into_iter()
            .find(|d| d.address == address.clone());

        device
    }

    pub async fn connect_device(
        &mut self,
        address: DeviceAddress,
    ) -> Result<bool, &'static str> {
        let mut device = self.get_device(address.clone()).await.unwrap();
        device.pair().await;
        let rx = device.connect().await;
        self.devices_connected
            .insert(address.clone(), device.clone());

        std::thread::spawn(move || {
            for r in rx.unwrap() {
                println!("{}", r);
            }
        });

        Ok(true)
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
