use std::time::{Duration, Instant};
use std::sync::Mutex;

use crate::framework_bluetooth::Communication;

use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, Result, Responder};
use actix_web_actors::ws;

use crate::framework_api::routes::bluetooth;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn scan_start(data: web::Data<Mutex<Communication>>) -> Result<impl Responder> {
    let mut communication = &data.lock().unwrap();
    let scan = communication.manager.start_scan().await;

    Ok(web::Json(scan))
}

pub struct MyScanWs {
    pub id: usize,
    hb: Instant,
    addr: Addr<bluetooth::ScanServerWS>
}

impl MyScanWs {
    pub fn new(srv: Addr<bluetooth::ScanServerWS>) -> Self {
        Self {
            id: 0,
            hb: Instant::now(),
            addr: srv,
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                act.addr.do_send(bluetooth::ScanDisconnect { 
                    id: act.id
                });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for MyScanWs {
    type Context = ws::WebsocketContext<Self>;

    // Start the heartbeat process for this connection
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(bluetooth::ScanConnect {
                addr: addr.recipient()
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
        self.addr.do_send(bluetooth::ScanDisconnect { 
            id: self.id
        });
        Running::Stop
    }
}

impl Handler<bluetooth::ScanServerMessage> for MyScanWs {
    type Result = ();

    fn handle(&mut self, msg: bluetooth::ScanServerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyScanWs {
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

                
            }
            // Close will close the socket
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {
                ctx.stop()
            },
        }
    }
}


pub async fn scan_stream(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<bluetooth::ScanServerWS>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyScanWs::new(srv.get_ref().clone()), &req, stream);
    // println!("{:?}", resp);
    resp
}
