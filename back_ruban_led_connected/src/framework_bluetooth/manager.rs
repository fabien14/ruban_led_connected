use std::collections::HashMap;
use bluer::{Adapter, Session};
use futures::pin_mut;
use tokio::time;

#[derive(Clone)]
pub struct Manager {
    pub devices_connected: HashMap<[u8; 6], String>, 
    session: Session,
    adapter: Adapter,
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

    pub async fn get_devices(&self) {
        let device_addresses_return = self.adapter.device_addresses().await;
        let device_addresses = match device_addresses_return {
            Ok(device_addresses) => device_addresses,
            Err(error) => panic!("Adresse not found : {:?}", error),
        };

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