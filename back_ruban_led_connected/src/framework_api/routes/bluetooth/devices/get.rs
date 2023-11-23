use crate::framework_bluetooth::{Communication, DeviceAddress, DevicesFilters};

use actix_web::{web, Responder, Result};

use std::sync::Mutex;

pub async fn devices(data: web::Data<Mutex<Communication>>, filter: web::Query<DevicesFilters>) -> Result<impl Responder> {
    let manager_bluetooth = &data.lock().unwrap();
    let devices = manager_bluetooth.manager.get_devices(filter.into_inner()).await.unwrap();

    Ok(web::Json(devices))
}

pub async fn device(
    data: web::Data<Mutex<Communication>>,
    device_address_path: web::Path<DeviceAddress>,
) -> Result<impl Responder> {
    let manager_bluetooth = &data.lock().unwrap();
    let device = manager_bluetooth
        .manager
        .get_device(device_address_path.clone())
        .await
        .unwrap();

    Ok(web::Json(device))
}
