use std::collections::HashMap;
use bluer::{Adapter, Device, Session, AdapterEvent, Address};
use futures::{pin_mut, StreamExt};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Manager {
    pub devices_connected: HashMap<[u8; 6], String>, 
    session: Session,
    adapter: Adapter,
    device_found: Arc<Mutex<Vec<Address>>>,
}

impl Manager {
    pub async fn new() -> Manager {
        let session = match Session::new().await {
            Ok(session) => session,
            Err(e) => panic!("New session error : {:?}", e),
        };
        let adapter = match session.default_adapter().await {
            Ok(adapter) => adapter,
            Err(e) => panic!("New adapter error : {:?}", e),
        };
        
        let _ = adapter.set_powered(true).await;

        Manager { 
            devices_connected: HashMap::new(), 
            session: session.clone(), 
            adapter: adapter.clone(),
            device_found: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn start_scan(&self) {
        let device_found_clone = Arc::clone(&(self.device_found));
        let cl = self.adapter.clone();

        tokio::spawn(async move {
            //let mut device_found_list = device_found_clone.lock().unwrap();
            let discover = cl.discover_devices().await.unwrap();
            pin_mut!(discover);

            let mut compter = 0;
            while compter < 10 {

                let evt = discover.next().await.unwrap();
                
                match evt {
                    AdapterEvent::DeviceAdded(addr) => {
                        //(*device_found_list).push(addr);
                        println!("Device added {addr}");
                    },
                    AdapterEvent::DeviceRemoved(addr) => {
                        println!("Device removed {addr}");
                    },
                    AdapterEvent::PropertyChanged(addr) => { 
                        println!("Device PropertyChanged {:?}", addr);
                    },
                }
                compter += 1;
            }
        });
        
        
        println!("Stopping discovery");
    }

    pub async fn get_devices(&self) {
        let device_found_clone = Arc::clone(&(self.device_found));
        let device_found_list = device_found_clone.lock().unwrap();
        let bis: Vec<Address> = (*device_found_list).clone();

        for d in bis {
            println!("{:?}", d);
        }

        let device_addresses_return = self.adapter.device_addresses().await;
        let device_addresses = match device_addresses_return {
            Ok(device_addresses) => device_addresses,
            Err(error) => panic!("Qdresse found : {:?}", error),
        };

        let mut target_device : Option<Device> = None;
        for device_addresse in device_addresses {
            println!("{:?}", device_addresse);

            let device = self.adapter.device(device_addresse);
            let option_name = match device {
                Ok(d) => d.name().await.unwrap(),
                Err(e) => {
                    println!("{:?}", e);
                    None
                },
            };

            let name = match option_name {
                Some(n) => n,
                None => String::from("NA"),
            };

            println!("Device found: {device_addresse} {name}");
        }
    }
}