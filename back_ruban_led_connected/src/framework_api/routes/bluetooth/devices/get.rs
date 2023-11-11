use crate::framework_bluetooth::{Communication, DeviceAddress, DevicesFilters};

use actix_web::{web, Responder, Result};

pub async fn devices(data: web::Data<Communication>, filter: web::Query<DevicesFilters>) -> Result<impl Responder> {
    let manager_bluetooth = &data;
    let devices = manager_bluetooth.manager.get_devices(Some(filter.into_inner())).await.unwrap();

    Ok(web::Json(devices))
}

pub async fn device(
    data: web::Data<Communication>,
    device_address_path: web::Path<DeviceAddress>,
) -> Result<impl Responder> {
    let manager_bluetooth = &data;
    let device = manager_bluetooth
        .manager
        .get_device(device_address_path.clone())
        .await
        .unwrap();

    Ok(web::Json(device))
}
