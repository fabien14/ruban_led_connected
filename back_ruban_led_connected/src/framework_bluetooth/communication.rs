use crate::framework_bluetooth::{DeviceAddress, Manager};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

#[derive(Clone)]
pub struct Communication {
    pub manager: Manager,
}

impl Communication {
    pub async fn new() -> Self {
        let manager = Manager::new().await;

        let communication = Self { manager: manager };

        communication
    }

    pub fn get_cannaux(&mut self) -> (SyncSender<String>, Receiver<String>) {
        let (tx_in_externe, rx_in_intern) = sync_channel(1);
        // exterieur => bleutooth manager
        let (tx_out_intern, rx_out_externe) = sync_channel(1);
        // bleutooth manager = > exterieur

        let mut comm = self.clone();

        let _ = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                comm.receiver(rx_in_intern, tx_out_intern).await;
            });
        });

        (tx_in_externe, rx_out_externe)
    }

    async fn send(&self, tx_out_intern: SyncSender<String>, message: String) {
        let tx = tx_out_intern.clone();
        tx.send(message).unwrap();
    }

    async fn receiver(&mut self, rx_in_intern: Receiver<String>, tx_out_intern: SyncSender<String>) {
        for rx_msg in rx_in_intern {
            println!("{}", rx_msg);
            if rx_msg.starts_with("connect ") {
                let addr = rx_msg.replace("connect ", "");
                let device_address = DeviceAddress(addr);
                match self.manager.devices_connected.get(&device_address) {
                    Some(_) => continue,
                    None => ()
                }
                
                let _ = self.manager.connect_device(device_address).await;
                
            } else if rx_msg.starts_with("send ") {
                let rx_message_without_type = rx_msg.replace("send ", "");
                let mut iter_msg = rx_message_without_type.split_whitespace();
                let addr = iter_msg.next();
                let message = iter_msg.next();
                let device_address = DeviceAddress(addr.unwrap().to_string());
                self.manager.send_device(device_address, message.unwrap().to_string()).await;
            }
        }
    }
}
