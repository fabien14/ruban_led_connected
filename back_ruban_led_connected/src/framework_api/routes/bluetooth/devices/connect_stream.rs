use crate::framework_bluetooth::{Communication, DeviceAddress};
use std::collections::HashMap;

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub address_device: DeviceAddress
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub address_device: DeviceAddress
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BluetoothMessage {
    pub msg: String,
    pub address_device: DeviceAddress
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    pub msg: String,
    pub address_device: DeviceAddress
}

#[derive(Clone)]
pub struct BluetoothServerWS {
    sessions: HashMap<DeviceAddress, HashMap<usize, Recipient<Message>>>,
    rng: ThreadRng,
    app_blue: Arc<Mutex<Communication>>,
    sender_blue: Option<SyncSender<String>>,
}

impl BluetoothServerWS {
    pub fn new(app_blue: Arc<Mutex<Communication>>) -> Self {
        Self {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
            app_blue: app_blue,
            sender_blue: None,
        }
    }
}

impl BluetoothServerWS {
    fn send_message(&mut self, address_device:DeviceAddress, message: &str, skip_id: usize) {
        match self.sessions.get_mut(&address_device) {
            Some(session) => {
                for (id, addr) in session {
                    if *id != skip_id {
                        addr.do_send(Message(message.to_owned()));
                    }
                }
            },
            None => ()
        }
    }
}

impl Actor for BluetoothServerWS {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("ici");
        let mut communication_blue = self.app_blue.lock().unwrap();
        let (tx_in_externe, rx_out_externe) = communication_blue.get_cannaux();
        self.sender_blue = Some(tx_in_externe.clone());
        let serveur = ctx.address();

        println!("{:?}", communication_blue.send_to_manager);

        let _ = std::thread::spawn(move || {
            println!("ici thread");
            for rx in rx_out_externe {
                println!("rx message {}", rx);
                let mut iter_message = rx.split_whitespace();
                let address_device = iter_message.next();
                let msg = iter_message.next();
                serveur.do_send(BluetoothMessage { 
                    address_device: DeviceAddress(address_device.unwrap().to_string()),
                    msg: msg.unwrap().to_string()
                });
            }
            println!("ici thread end");
        });
        println!("ici finiii");
    }
}

impl Handler<Connect> for BluetoothServerWS {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<usize>();

        match self.sessions.get_mut(&msg.address_device) {
            Some(session) => {
                session.insert(id, msg.addr.to_owned());
            },
            None => {
                let mut session_new = HashMap::new();
                session_new.insert(id, msg.addr.to_owned());
                self.sessions.insert(msg.address_device.clone(), session_new);
            }
        }

        let message = format!("connect {}", msg.address_device);
        self.sender_blue.clone().unwrap().send(message).unwrap();

        // send id back
        id
    }
}

impl Handler<Disconnect> for BluetoothServerWS {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        match self.sessions.get_mut(&msg.address_device) {
            Some(session) => {
                session.remove(&msg.id).is_some();
            },
            None => ()
        }
    }
}

impl Handler<BluetoothMessage> for BluetoothServerWS {
    type Result = ();

    fn handle(&mut self, msg: BluetoothMessage, _: &mut Context<Self>) {
        // tous les messages utilisateur passe par ici
        // envoyer via self.sender le message recu au communicateur bluetooth
        self.send_message(msg.address_device, msg.msg.as_str(), 0);
    }
}

impl Handler<ClientMessage> for BluetoothServerWS {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        // tous les messages utilisateur passe par ici
        // envoyer via self.sender le message recu au communicateur bluetooth
        println!("{:?}", msg);
        self.send_message(msg.address_device.clone(), msg.msg.as_str(), 0);

        let message = format!("send {} {}", msg.address_device, msg.msg);
        println!("{:?}", message);
        self.sender_blue.clone().unwrap().send(message).unwrap();
    }
}
