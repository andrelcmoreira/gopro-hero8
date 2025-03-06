use std::collections::BTreeSet;
use std::io::Read;

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter,
                    Characteristic, CharPropFlags};
use btleplug::platform::{Adapter, Manager, Peripheral};
use log::{info, debug};
use uuid::Uuid;

use crate::error::AppError;

async fn get_str_prop(cam: &Peripheral, prop: &str, service: &str) -> String {
    let ch = Characteristic {
        uuid: Uuid::parse_str(prop).unwrap(),
        service_uuid: Uuid::parse_str(service).unwrap(),
        properties: CharPropFlags::READ,
        descriptors: BTreeSet::new()
    };
    let val = cam.read(&ch)
        .await
        .unwrap_or(vec![]);

    debug!("raw property: {:?}", val.bytes());

    // TODO: improve this
    match String::from_utf8(val) {
        Ok(val) => {
            if val != "\0" {
                val
            } else {
                "".to_string()
            }
        },
        Err(_) => "".to_string()
    }
}

async fn get_int_prop(cam: &Peripheral, prop: &str, service: &str) -> u8 {
    get_str_prop(&cam, prop, service)
        .await
        .chars()
        .nth(0)
        .unwrap_or('0') as u8
}

// TODO: maybe separate this function to a separate module
pub async fn get_hw_revision(cam: &Peripheral) -> String {
    const PROP: &str = "00002a27-0000-1000-8000-00805f9b34fb";
    const SERVICE: &str = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_str_prop(&cam, PROP, SERVICE)
        .await
}

// TODO: maybe separate this function to a separate module
pub async fn get_fw_revision(cam: &Peripheral) -> String {
    const PROP: &str = "00002a26-0000-1000-8000-00805f9b34fb";
    const SERVICE: &str = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_str_prop(&cam, PROP, SERVICE)
        .await
}

// TODO: maybe separate this function to a separate module
pub async fn get_sw_revision(cam: &Peripheral) -> String {
    const PROP: &str = "00002a28-0000-1000-8000-00805f9b34fb";
    const SERVICE: &str = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_str_prop(&cam, PROP, SERVICE)
        .await
}

// TODO: maybe separate this function to a separate module
pub async fn get_serial_number(cam: &Peripheral) -> String {
    const PROP: &str = "00002a25-0000-1000-8000-00805f9b34fb";
    const SERVICE: &str = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_str_prop(&cam, PROP, SERVICE)
        .await
}

// TODO: maybe separate this function to a separate module
pub async fn get_model_number(cam: &Peripheral) -> String {
    const PROP: &str = "00002a24-0000-1000-8000-00805f9b34fb";
    const SERVICE: &str = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_str_prop(&cam, PROP, SERVICE)
        .await
}

// TODO: maybe separate this function to a separate module
pub async fn get_manufacturer_name(cam: &Peripheral) -> String {
    const PROP: &str = "00002a29-0000-1000-8000-00805f9b34fb";
    const SERVICE: &str = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_str_prop(&cam, PROP, SERVICE)
        .await
}

// TODO: maybe separate this function to a separate module
pub async fn get_wifi_ssid(cam: &Peripheral) -> String {
    const PROP: &str = "b5f90002-aa8d-11e3-9046-0002a5d5c51b";
    const SERVICE: &str = "b5f90001-aa8d-11e3-9046-0002a5d5c51b";

    return get_str_prop(&cam, PROP, SERVICE)
        .await
}

// TODO: maybe separate this function to a separate module
pub async fn get_wifi_password(cam: &Peripheral) -> String {
    const PROP: &str = "b5f90003-aa8d-11e3-9046-0002a5d5c51b";
    const SERVICE: &str = "b5f90001-aa8d-11e3-9046-0002a5d5c51b";

    return get_str_prop(&cam, PROP, SERVICE)
        .await
}

// TODO: maybe separate this function to a separate module
pub async fn get_battery_level(cam: &Peripheral) -> u8 {
    const PROP: &str = "00002a19-0000-1000-8000-00805f9b34fb";
    const SERVICE: &str = "0000180f-0000-1000-8000-00805f9b34fb";

    return get_int_prop(&cam, PROP, SERVICE)
        .await
}

// TODO: maybe separate this function to a separate module
pub async fn get_tx_power_level(cam: &Peripheral) -> u8 {
    const PROP: &str = "00002a07-0000-1000-8000-00805f9b34fb";
    const SERVICE: &str = "00001804-0000-1000-8000-00805f9b34fb";

    return get_int_prop(&cam, PROP, SERVICE)
        .await
}

// TODO: maybe separate this function to a separate module
pub async fn get_characteristic_cfg(cam: &Peripheral) -> String {
    const PROP: &str = "b5f90005-aa8d-11e3-9046-0002a5d5c51b";
    const SERVICE: &str = "b5f90001-aa8d-11e3-9046-0002a5d5c51b";

    return get_str_prop(&cam, PROP, SERVICE)
        .await
}

// TODO: discover what this value represents
// TODO: maybe separate this function to a separate module
pub async fn get_unknown_field(cam: &Peripheral) -> String {
    const PROP: &str = "b5f90006-aa8d-11e3-9046-0002a5d5c51b";
    const SERVICE: &str = "b5f90001-aa8d-11e3-9046-0002a5d5c51b";

    return get_str_prop(&cam, PROP, SERVICE)
        .await
}

pub async fn get_adapter() -> Result<Adapter, AppError> {
    let mgr = Manager::new()
        .await
        .map_err(|_| AppError::BluetoothNotAvailable);
    let adapter = mgr?.adapters()
        .await
        .map_err(|_| AppError::BluetoothNotAvailable)
        ?.into_iter()
        .nth(0)
        .unwrap();

    Ok(adapter)
}

pub async fn connect_to_cam(adapter: &Adapter) -> Result<Peripheral, AppError> {
    if let Err(_) = adapter.start_scan(ScanFilter::default()).await {
        return Err(AppError::DeviceLookupError)
    }

    let cam = match find_camera(&adapter).await {
        Some(c) => c,
        None => return Err(AppError::CameraNotFound)
    };

    let is_connected = cam.is_connected()
        .await
        .map_err(|_| AppError::ConnectionFailure)?;
    if ! is_connected {
        info!("trying to connect to the camera...");
        if let Err(_) = cam.connect().await {
            return Err(AppError::ConnectionFailure)
        }
    }

    debug!("is connected? {is_connected}");

    if let Err(_) = cam.discover_services().await {
        return Err(AppError::ConnectionFailure)
    }

    Ok(cam)
}

async fn find_camera(adapter: &Adapter) -> Option<Peripheral> {
    let peripherals = adapter.peripherals()
        .await
        .unwrap_or(vec![]);

    for entry in peripherals {
        let properties = entry.properties()
            .await
            .unwrap();

        if let Some(prop) = properties {
            let dev = prop.local_name
                .unwrap_or("".to_string());

            debug!("device: {dev}");

            if ! dev.is_empty() && dev.contains("GoPro") {
                return Some(entry)
            }
        }
    }

    None
}
