use std::sync::Mutex;
use crate::framework_bluetooth::Communication;

use actix_web::{web, Result, Responder};

pub async fn scan(data: web::Data<Mutex<Communication>>) -> Result<impl Responder> {
    let communication = &data.lock().unwrap();
    let scan = communication.manager.scan().await;

    Ok(web::Json(scan))
}