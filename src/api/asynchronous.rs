use crate::data::factory_info::FactoryInfo;
use crate::data::status_info::StatusInfo;
use crate::data::wifi_info::WifiInfo;
use crate::error::AppError;
use crate::protocol::ble;

pub async fn get_wifi_info() -> Result<WifiInfo, AppError> {
    let adapter = ble::get_adapter().await?;
    let cam = ble::connect_to_cam(&adapter).await?;
    let info = WifiInfo::new(
        ble::get_wifi_ssid(&cam).await,
        ble::get_wifi_password(&cam).await
    );

    Ok(info)
}

pub async fn get_factory_info() -> Result<FactoryInfo, AppError> {
    let adapter = ble::get_adapter().await?;
    let cam = ble::connect_to_cam(&adapter).await?;
    let info = FactoryInfo::new(
        ble::get_hw_revision(&cam).await,
        ble::get_fw_revision(&cam).await,
        ble::get_sw_revision(&cam).await,
        ble::get_serial_number(&cam).await,
        ble::get_model_number(&cam).await,
        ble::get_manufacturer_name(&cam).await
    );

    Ok(info)
}

pub async fn get_status_info() -> Result<StatusInfo, AppError> {
    let adapter = ble::get_adapter().await?;
    let cam = ble::connect_to_cam(&adapter).await?;
    let info = StatusInfo::new(
        ble::get_battery_level(&cam).await,
        ble::get_tx_power_level(&cam).await
    );

    Ok(info)
}
