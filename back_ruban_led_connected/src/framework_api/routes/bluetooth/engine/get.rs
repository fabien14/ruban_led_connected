use crate::framework_bluetooth::Communication;

use actix_web::{web, Result, Responder};

pub async fn scan(data: web::Data<Communication>) -> Result<impl Responder> {
    let communication = &data;
    let manager_bluetooth = communication.manager.clone();
    let scan = manager_bluetooth.scan().await;

    Ok(web::Json(scan))
}