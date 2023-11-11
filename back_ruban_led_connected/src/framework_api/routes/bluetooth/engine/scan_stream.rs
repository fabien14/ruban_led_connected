use crate::framework_bluetooth::Communication;
use std::collections::HashMap;

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::time::Duration;
use std::sync::{ Arc, Mutex };

#[derive(Message)]
#[rtype(result = "()")]
pub struct ScanServerMessage(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct ScanConnect {
    pub addr: Recipient<ScanServerMessage>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ScanDisconnect {
    pub id: usize
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ScanMessage {
    pub msg: String
}

#[derive(Clone)]
pub struct ScanServerWS {
    sessions: HashMap<usize, Recipient<ScanServerMessage>>,
    rng: ThreadRng,
    app_blue: Communication,
    threads_get_devices_started: Arc<Mutex<HashMap<std::thread::ThreadId, bool>>>
}

impl ScanServerWS {

    pub fn new(app_blue: Communication) -> Self {
        Self {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
            app_blue: app_blue,
            threads_get_devices_started: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn start_get_devices(&self, addr_server: Addr<ScanServerWS>) {
        let blue_manager = self.app_blue.manager.clone();
        let threads_get_devices_started_ref = Arc::clone(&self.threads_get_devices_started);
        
        let _ = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {

                let mut continue_get_devices = true;
                let thread_id = std::thread::current().id();
                
                while continue_get_devices {

                    let devices = blue_manager.get_devices(None).await;
                    match devices {
                        Some(devices) => {
                            addr_server.do_send(ScanMessage { 
                                msg: devices.to_json_string()
                            });
                        },
                        _ => continue
                    }
                    let trois_second = Duration::from_secs(3);
                    std::thread::sleep(trois_second);

                    let mut threads_get_devices_started_ref_val = threads_get_devices_started_ref.lock().unwrap();

                    match threads_get_devices_started_ref_val.get(&thread_id) {
                        Some(&continue_thread_get_devices) => {
                            if !continue_thread_get_devices {
                                threads_get_devices_started_ref_val.remove(&thread_id);
                                continue_get_devices = false;
                            }
                        },
                        None => {
                            if threads_get_devices_started_ref_val.clone().into_iter().any(|(_, v)| v == true) {
                                continue_get_devices = false;
                            }
                            else {
                                threads_get_devices_started_ref_val.insert(thread_id, true);
                            }
                        },
                    }
                }
            });
        });
    }

    pub fn stop_get_devices(&self) {
        let mut get_devices_started_ref_val = self.threads_get_devices_started.lock().unwrap();
        for val in get_devices_started_ref_val.values_mut() {
            *val = false;
        }
    }

}

impl ScanServerWS {
    fn send_message(&mut self, message: &str, skip_id: usize) {
        for (id, addr) in &self.sessions {
            if *id != skip_id {
                addr.do_send(ScanServerMessage(message.to_owned()));
            }
        }
    }
}

impl Actor for ScanServerWS {
    type Context = Context<Self>;
}

impl Handler<ScanConnect> for ScanServerWS {
    type Result = usize;

    fn handle(&mut self, msg: ScanConnect, ctx: &mut Context<Self>) -> Self::Result {
        self.start_get_devices(ctx.address());
        let id = self.rng.gen::<usize>();

        self.sessions.insert(id, msg.addr.to_owned());
        // send id back
        id
    }
}

impl Handler<ScanDisconnect> for ScanServerWS {
    type Result = ();

    fn handle(&mut self, msg: ScanDisconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id).is_some();
        if self.sessions.keys().len() == 0 {
            self.stop_get_devices();
        }
    }
}

impl Handler<ScanMessage> for ScanServerWS {
    type Result = ();

    fn handle(&mut self, msg: ScanMessage, _: &mut Context<Self>) {
        // tous les messages utilisateur passe par ici
        // envoyer via self.sender le message recu au communicateur bluetooth
        self.send_message(msg.msg.as_str(), 0);
    }
}
