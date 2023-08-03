use crate::configuration::Settings;
use crate::framework_api::{scan, devices, device};
use crate::framework_bluetooth::Manager;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use anyhow;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings, app_blue: Manager) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            configuration.application.base_url,
            app_blue,
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
    app_blue: Manager,
) -> Result<Server, anyhow::Error> {
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let app_data_blue = web::Data::new(app_blue.clone());

    let server = HttpServer::new(move || {
        App::new()
            .route("/bluetooth/scan", web::post().to(scan))
            .route("/bluetooth/devices", web::get().to(devices))
            .route("/bluetooth/devices/{device_address}", web::get().to(device))
            .app_data(base_url.clone())
            .app_data(app_data_blue.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}