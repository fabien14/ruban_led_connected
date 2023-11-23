use crate::configuration::Settings;
use crate::framework_api::{connect, stream, device, devices, scan, scan_start, scan_stream, BluetoothServerWS, ScanServerWS};
use crate::framework_bluetooth::Communication;

use actix_cors::Cors;
use actix::{Actor, Addr};
use actix_web::dev::Server;
use actix_web::{guard, web, App, HttpServer, HttpResponse};
use anyhow;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(
        configuration: Settings,
        app_blue: Arc<Mutex<Communication>>,
    ) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let ws_bluetooth_devices = BluetoothServerWS::new(Arc::clone(&app_blue)).start();
        let ws_scan_devices = ScanServerWS::new(Arc::clone(&app_blue)).start();
        let server = run(
            listener,
            configuration.application.base_url,
            app_blue,
            ws_bluetooth_devices,
            ws_scan_devices
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

#[derive(Debug)]
pub struct ApplicationBaseUrl(pub String);

async fn run(
    listener: TcpListener,
    base_url: String,
    app_blue: Arc<Mutex<Communication>>,
    ws_bluetooth_devices: Addr<BluetoothServerWS>,
    ws_scan_devices: Addr<ScanServerWS>
) -> Result<Server, anyhow::Error> {
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let app_data_blue = web::Data::from(Arc::clone(&app_blue));
    let bluetooth_server = web::Data::new(ws_bluetooth_devices.clone());
    let scan_server = web::Data::new(ws_scan_devices.clone());

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")})
            .allowed_methods(vec!["GET", "POST", "PATCH", "PUT", "DELETE", "OPTIONS"])
            .allow_any_header();

        App::new()
            .wrap(cors)
            .route("/bluetooth/scan", web::get().to(scan))
            .route("/bluetooth/scan", web::post().to(scan_start))
            .route("/bluetooth/scan-stream", web::get().to(scan_stream))
            .route("/bluetooth/devices", web::get().to(devices))
            .route("/bluetooth/devices/{device_address}", web::get().to(device))
            .route(
                "/bluetooth/devices/{device_address}/connect",
                web::get().to(connect),
            )
            .route(
                "/bluetooth/devices/{device_address}/stream",
                web::get().to(stream),
            )
            .app_data(bluetooth_server.clone())
            .app_data(scan_server.clone())
            .app_data(base_url.clone())
            .app_data(web::Data::clone(&app_data_blue))
            .default_service(web::route().guard(guard::Options()).to(|| HttpResponse::Ok()))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
