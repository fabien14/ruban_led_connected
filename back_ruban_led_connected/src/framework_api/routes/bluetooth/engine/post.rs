use crate::framework_bluetooth::Communication;

use actix_web::{web, HttpResponse, Responder, Result};

pub async fn scan(data: web::Data<Communication>) -> Result<impl Responder> {
    let manager_bluetooth = &data;
    manager_bluetooth.manager.start_scan().await;

    Ok(HttpResponse::Ok())
}
