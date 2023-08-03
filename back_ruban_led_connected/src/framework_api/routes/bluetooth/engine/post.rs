use crate::framework_bluetooth::Manager;

use actix_web::{web, Responder, Result, HttpResponse};


pub async fn scan(data: web::Data<Manager>) -> Result<impl Responder> {
    let manager_bluetooth = &data;
    manager_bluetooth.start_scan().await;

    Ok(HttpResponse::Ok())
}