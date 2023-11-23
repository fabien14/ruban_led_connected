use std::sync::Mutex;
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, Result, Responder};
use actix_web_actors::ws;

use crate::framework_api::routes::bluetooth;
use crate::framework_bluetooth::{DeviceAddress, Communication};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Define HTTP actor
pub struct MyWs {
    pub id: usize,
    hb: Instant,
    addr: Addr<bluetooth::BluetoothServerWS>,
    blue_device_addr: DeviceAddress
}

impl MyWs {
    pub fn new(srv: Addr<bluetooth::BluetoothServerWS>, device_address: DeviceAddress) -> Self {
        Self {
            id: 0,
            hb: Instant::now(),
            addr: srv,
            blue_device_addr: device_address
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                act.addr.do_send(bluetooth::Disconnect { 
                    id: act.id,
                    address_device: act.blue_device_addr.clone()
                });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    // Start the heartbeat process for this connection
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(bluetooth::Connect {
                addr: addr.recipient(),
                address_device: self.blue_device_addr.clone()
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(bluetooth::Disconnect { 
            id: self.id,
            address_device: self.blue_device_addr.clone()
        });
        Running::Stop
    }
}

impl Handler<bluetooth::Message> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: bluetooth::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("{:?}", msg);
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            // Text will echo any text received back to the client (for now)
            ws::Message::Text(text) => {
                //ctx.text(text)
                println!("bah");

                let m = text.trim();
                let msg = m.to_owned();

                self.addr
                    .do_send(bluetooth::ClientMessage { 
                        address_device: self.blue_device_addr.clone(),
                        id: self.id, 
                        msg: msg 
                    });
            }
            // Close will close the socket
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

pub async fn stream(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<bluetooth::BluetoothServerWS>>,
    device_address_path: web::Path<DeviceAddress>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs::new(srv.get_ref().clone(), device_address_path.clone()), &req, stream);
    println!("{:?}", resp);
    resp
}

pub async fn connect(device_address_path: web::Path<DeviceAddress>, data: web::Data<Mutex<Communication>>) -> Result<impl Responder> {
    let mut communication = &data.lock().unwrap();
    let message = format!("connect {}", device_address_path.to_owned());
    communication.send_to_manager(message);

    //let mut manager_bluetooth = communication.manager.clone();
    //let mut mana = &communication.manager;
    //let _ = mana.connect_device(device_address_path.to_owned()).await;

    Ok(HttpResponse::Ok())
}
