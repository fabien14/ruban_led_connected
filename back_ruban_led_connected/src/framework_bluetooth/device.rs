use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use bluer::rfcomm::stream::{OwnedReadHalf, OwnedWriteHalf};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::time::Duration;
use std::fmt;

const SCAN_DURATION: Duration = Duration::from_secs(20);

#[derive(Clone, Serialize, PartialEq, Eq, Debug)]
pub struct DeviceName(pub String);

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct DeviceAddress(pub String);

impl fmt::Display for DeviceAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct Device {
    pub name: DeviceName,
    pub address: DeviceAddress,
    pub paired: bool,
    pub connected: bool,
    #[serde(skip_serializing)]
    bluer_device: bluer::Device,
    #[serde(skip_serializing)]
    pub sender: Option<SyncSender<String>>,
}

impl Device {
    pub async fn from(bluer_device: bluer::Device) -> Self {
        let option_name = match bluer_device.name().await {
            Ok(n) => n,
            Err(_) => None,
        };

        let name = match option_name {
            Some(n) => n,
            None => String::from("NA"),
        };

        let paired = match bluer_device.is_paired().await {
            Ok(p) => p,
            Err(_) => false
        };

        let connected = match bluer_device.is_connected().await {
            Ok(c) => c,
            Err(_) => false
        };

        Self {
            name: DeviceName(name),
            address: DeviceAddress(bluer_device.address().to_string()),
            paired: paired,
            connected: connected,
            bluer_device: bluer_device,
            sender: None
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    pub async fn pair(&self) {
        let is_paired = self.bluer_device.is_paired().await.unwrap_or_default();
        if !is_paired {
            let _ = self.bluer_device.pair().await;
        }
    }

    pub async fn connect(&mut self) -> Result<Receiver<String>, &'static str> {
        let device_adresse = self.bluer_device.address();
        let sock_addr = bluer::rfcomm::SocketAddr::new(device_adresse, 1);
        println!("{}", sock_addr);
        let mut stream_socket = None;

        println!("    Connecting...");
        println!("dddddd");
        let mut retries = 2;
        while retries > 0 {
            stream_socket = match bluer::rfcomm::Stream::connect(sock_addr).await {
                Ok(stream_socket) => {
                    retries = 0;
                    println!("    Connected ");
                    Some(stream_socket)
                }
                Err(err) => {
                    println!("    Connect error: {}", &err);
                    retries -= 1;
                    None
                }
            };
        }

        match stream_socket {
            Some(stre) => {
                let (tx_send, rx_send) = sync_channel(1);
                let (tx_receive, rx_receive) = sync_channel(1);
                let device_for_read = self.clone();
                let device_for_write = self.clone();
                let _ = std::thread::spawn(move || {
                    println!("thread stream");
                    //let stream = stre;
                    let (rh, wh) = stre.into_split();

                    let read_stream = std::thread::spawn(move || {
                        let rt = tokio::runtime::Builder::new_multi_thread()
                            .enable_all()
                            .build()
                            .unwrap();
                        rt.block_on(async {
                            println!("thread device read");
                            device_for_read.receiver(rh, tx_receive).await;
                            println!("thread device read end");
                        });
                    });

                    let write_stream = std::thread::spawn(move || {
                        let rt = tokio::runtime::Builder::new_multi_thread()
                            .enable_all()
                            .build()
                            .unwrap();
                        rt.block_on(async {
                            println!("thread device write");
                            device_for_write.writer(wh, rx_send).await;
                            println!("thread device write end");
                        });
                    });

                    let _ = write_stream.join();
                    let _ = read_stream.join();
                });

                self.sender = Some(tx_send);
                return Ok(rx_receive);
            }
            _ => {
                return Err("Erreur de merde");
            }
        }
    }

    pub fn send(&self, message: String) {
        match self.sender.clone() {
            Some(channel_sender) => channel_sender.send(message).unwrap(),
            _ => ()
        }
    }

    async fn receiver(&self, mut rh: OwnedReadHalf, tx: SyncSender<String>) {
        // 1 receive data by rh
        // 2 send data in channel

        loop {
            let mut buffer = String::new();
            let len_string_read = rh.read_to_string(&mut buffer).await.unwrap();
            if len_string_read > 1 {
                tx.send(buffer).unwrap();
            }
        }
    }

    async fn writer(&self, mut wh: OwnedWriteHalf, rx: Receiver<String>) {
        // 1 receive data by channel
        // 2 send data in wh

        // wh.write_all("ssss".as_bytes()).await.expect("write failed");

        for msg in rx {
            wh.write_all(msg.as_bytes()).await.expect("write failed");
        }

        /*let tx = self.tx.clone();
        println!("{:?}", tx);
        println!("{}", message);
        match tx {
            Some(t) => t.send(message).unwrap(),
            _ => ()
        }*/
    }
}
