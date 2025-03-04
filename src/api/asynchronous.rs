use std::io::Error;

use crate::protocol::bluetooth::*;

use crate::data::factory_info::FactoryInfo;
use crate::data::status_info::StatusInfo;
use crate::data::wifi_info::WifiInfo;

pub async fn get_wifi_info() -> Result<WifiInfo, Error> {
    let adapter = get_adapter().await?;
    let cam = connect_to_cam(&adapter).await?;
    let info = WifiInfo::new(
        get_wifi_ssid(&cam).await,
        get_wifi_password(&cam).await
    );

    Ok(info)
}

pub async fn get_factory_info() -> Result<FactoryInfo, Error> {
    let adapter = get_adapter().await?;
    let cam = connect_to_cam(&adapter).await?;
    let info = FactoryInfo::new(
        get_hw_revision(&cam).await,
        get_fw_revision(&cam).await,
        get_sw_revision(&cam).await,
        get_serial_number(&cam).await,
        get_model_number(&cam).await,
        get_manufacturer_name(&cam).await
    );

    Ok(info)
}

pub async fn get_status_info() -> Result<StatusInfo, Error> {
    let adapter = get_adapter().await?;
    let cam = connect_to_cam(&adapter).await?;
    let info = StatusInfo::new(
        get_battery_level(&cam).await,
        get_tx_power_level(&cam).await
    );

    Ok(info)
}
