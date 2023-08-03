use crate::framework_bluetooth::{Manager, DeviceAddress};

use actix_web::{web, Result, Responder};

pub async fn devices(data: web::Data<Manager>) -> Result<impl Responder> {
    let manager_bluetooth = &data;
    let devices = manager_bluetooth.get_devices().await.unwrap();

    Ok(web::Json(devices))
}

pub async fn device(data: web::Data<Manager>, device_address_path: web::Path<DeviceAddress>) -> Result<impl Responder> {
    let manager_bluetooth = &data;
    let device = manager_bluetooth.get_device(device_address_path.clone()).await.unwrap();

    Ok(web::Json(device))
}