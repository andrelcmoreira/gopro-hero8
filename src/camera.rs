use std::io::Error;
use tokio::runtime::Runtime;
use crate::bluetooth::*;

#[derive(Debug)]
pub struct CamWifiInfo {
    wifi_ssid: String,
    wifi_password: String,
}

#[derive(Debug)]
pub struct CamStatusInfo {
    battery_level: u8,
    tx_power_level: u8,
//    characteristic_cfg: String,
//    unknown_field: String // TODO: rename it
}

#[derive(Debug)]
pub struct CamFactoryInfo {
    hw_revision: String,
    fw_revision: String,
    sw_revision: String,
    serial_number: String,
    model_number: String,
    manufacturer_name: String,
}

pub fn get_cam_factory_info() -> Result<CamFactoryInfo, Error> {
    Runtime::new()
        .unwrap()
        .block_on(
            async {
                let info = get_cam_factory_info_async()
                    .await?;

                Ok(info)
            }
        )
}

pub fn get_cam_wifi_info() -> Result<CamWifiInfo, Error> {
    Runtime::new()
        .unwrap()
        .block_on(
            async {
                let info = get_cam_wifi_info_async()
                    .await?;

                Ok(info)
            }
        )
}

pub fn get_cam_status_info() -> Result<CamStatusInfo, Error> {
    Runtime::new()
        .unwrap()
        .block_on(
            async {
                let info = get_cam_status_info_async()
                    .await?;

                Ok(info)
            }
        )
}

// TODO: handle errors
pub async fn get_cam_wifi_info_async() -> Result<CamWifiInfo, Error> {
    let adapter = get_bt_adapter()
        .await
        .unwrap();
    let cam = connect_to_cam(&adapter)
        .await
        .unwrap();
    let info = CamWifiInfo {
        wifi_ssid: get_wifi_ssid(&cam).await,
        wifi_password: get_wifi_password(&cam).await
    };

    Ok(info)
}

// TODO: handle errors
pub async fn get_cam_factory_info_async() -> Result<CamFactoryInfo, Error> {
    let adapter = get_bt_adapter()
        .await
        .unwrap();
    let cam = connect_to_cam(&adapter)
        .await
        .unwrap();
    let info = CamFactoryInfo {
        hw_revision: get_hw_revision(&cam).await,
        fw_revision: get_fw_revision(&cam).await,
        sw_revision: get_sw_revision(&cam).await,
        serial_number: get_serial_number(&cam).await,
        model_number: get_model_number(&cam).await,
        manufacturer_name: get_manufacturer_name(&cam).await
    };

    Ok(info)
}

// TODO: handle errors
pub async fn get_cam_status_info_async() -> Result<CamStatusInfo, Error> {
    let adapter = get_bt_adapter()
        .await
        .unwrap();
    let cam = connect_to_cam(&adapter)
        .await
        .unwrap();
    let info = CamStatusInfo {
        battery_level: get_battery_level(&cam).await,
        tx_power_level: get_tx_power_level(&cam).await
    };

    Ok(info)
}
