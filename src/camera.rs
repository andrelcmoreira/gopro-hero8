use std::io::Error;

use tokio::runtime::Runtime;

use crate::bluetooth::*;

#[derive(Debug)]
pub struct CameraInfo {
    hw_revision: String,
    fw_revision: String,
    sw_revision: String,
    serial_number: String,
    model_number: String,
    manufacturer_name: String,
    wifi_ssid: String,
    wifi_password: String,
    battery_level: String,
    tx_power_level: String,
    characteristic_cfg: String,
    unknown_field: String
}

// TODO: handle errors
pub fn get_camera_info() -> Result<CameraInfo, Error> {
    Runtime::new()
        .unwrap()
        .block_on(
            async {
                let adapter = get_bt_adapter()
                    .await
                    .unwrap();
                let cam = connect_to_cam(&adapter)
                    .await
                    .unwrap();
                let info = CameraInfo {
                    hw_revision: get_hw_revision(&cam).await,
                    fw_revision: get_fw_revision(&cam).await,
                    sw_revision: get_sw_revision(&cam).await,
                    serial_number: get_serial_number(&cam).await,
                    model_number: get_model_number(&cam).await,
                    manufacturer_name: get_manufacturer_name(&cam).await,
                    wifi_ssid: get_wifi_ssid(&cam).await,
                    wifi_password: get_wifi_password(&cam).await,
                    battery_level: get_battery_level(&cam).await,
                    tx_power_level: get_tx_power_level(&cam).await,
                    characteristic_cfg: get_characteristic_cfg(&cam).await,
                    unknown_field: get_unknown_field(&cam).await
                };

                Ok(info)
            }
        )
}
