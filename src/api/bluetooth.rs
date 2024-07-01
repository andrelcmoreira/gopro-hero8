use std::collections::BTreeSet;
use std::io::Read;

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter,
                    Characteristic, CharPropFlags};
use btleplug::platform::{Adapter, Manager, Peripheral};
use btleplug::Error;
use log::{info, debug};
use uuid::Uuid;

async fn get_str_prop(cam: &Peripheral, prop: &str, service: &str) -> String {
    let ch = Characteristic {
        uuid: Uuid::parse_str(prop).unwrap(),
        service_uuid: Uuid::parse_str(service).unwrap(),
        properties: CharPropFlags::READ,
        descriptors: BTreeSet::new()
    };
    let val = cam
        .read(&ch)
        .await
        .unwrap();

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

pub async fn get_int_prop(cam: &Peripheral, prop: &str, service: &str) -> u8 {
    get_str_prop(&cam, prop, service)
        .await
        .chars()
        .nth(0)
        .unwrap() as u8
}

pub async fn get_hw_revision(cam: &Peripheral) -> String {
    let prop = "00002a27-0000-1000-8000-00805f9b34fb";
    let service = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_str_prop(&cam, prop, service)
        .await
}

pub async fn get_fw_revision(cam: &Peripheral) -> String {
    let prop = "00002a26-0000-1000-8000-00805f9b34fb";
    let service = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_str_prop(&cam, prop, service)
        .await
}

pub async fn get_sw_revision(cam: &Peripheral) -> String {
    let prop = "00002a28-0000-1000-8000-00805f9b34fb";
    let service = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_str_prop(&cam, prop, service)
        .await
}

pub async fn get_serial_number(cam: &Peripheral) -> String {
    let prop = "00002a25-0000-1000-8000-00805f9b34fb";
    let service = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_str_prop(&cam, prop, service)
        .await
}

pub async fn get_model_number(cam: &Peripheral) -> String {
    let prop = "00002a24-0000-1000-8000-00805f9b34fb";
    let service = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_str_prop(&cam, prop, service)
        .await
}

pub async fn get_manufacturer_name(cam: &Peripheral) -> String {
    let prop = "00002a29-0000-1000-8000-00805f9b34fb";
    let service = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_str_prop(&cam, prop, service)
        .await
}

pub async fn get_wifi_ssid(cam: &Peripheral) -> String {
    let prop = "b5f90002-aa8d-11e3-9046-0002a5d5c51b";
    let service = "b5f90001-aa8d-11e3-9046-0002a5d5c51b";

    return get_str_prop(&cam, prop, service)
        .await
}

pub async fn get_wifi_password(cam: &Peripheral) -> String {
    let prop = "b5f90003-aa8d-11e3-9046-0002a5d5c51b";
    let service = "b5f90001-aa8d-11e3-9046-0002a5d5c51b";

    return get_str_prop(&cam, prop, service)
        .await
}

pub async fn get_battery_level(cam: &Peripheral) -> u8 {
    let prop = "00002a19-0000-1000-8000-00805f9b34fb";
    let service = "0000180f-0000-1000-8000-00805f9b34fb";

    return get_int_prop(&cam, prop, service)
        .await
}

pub async fn get_tx_power_level(cam: &Peripheral) -> u8 {
    let prop = "00002a07-0000-1000-8000-00805f9b34fb";
    let service = "00001804-0000-1000-8000-00805f9b34fb";

    return get_int_prop(&cam, prop, service)
        .await
}

pub async fn get_characteristic_cfg(cam: &Peripheral) -> String {
    let prop = "b5f90005-aa8d-11e3-9046-0002a5d5c51b";
    let service = "b5f90001-aa8d-11e3-9046-0002a5d5c51b";

    return get_str_prop(&cam, prop, service)
        .await
}

// TODO: discover what this value represents
pub async fn get_unknown_field(cam: &Peripheral) -> String {
    let prop = "b5f90006-aa8d-11e3-9046-0002a5d5c51b";
    let service = "b5f90001-aa8d-11e3-9046-0002a5d5c51b";

    return get_str_prop(&cam, prop, service)
        .await
}

pub async fn get_adapter() -> Result<Adapter, Error> {
    let mgr = Manager::new()
        .await?;
    let adapter = mgr
        .adapters()
        .await?
        .into_iter()
        .nth(0)
        .unwrap();

    Ok(adapter)
}

pub async fn connect_to_cam(adapter: &Adapter) -> Result<Peripheral, Error> {
    adapter.start_scan(ScanFilter::default())
        .await?;

    let cam = find_camera(&adapter)
        .await
        .unwrap();
    if ! cam.is_connected().await? {
        info!("trying to connect...");
        cam.connect()
            .await?
    }

    debug!("is connected? {}", cam.is_connected().await?);

    cam.discover_services()
        .await?;

    Ok(cam)
}

async fn find_camera(adapter: &Adapter) -> Option<Peripheral> {
    let devices = adapter
        .peripherals()
        .await
        .unwrap();

    for entry in devices {
        let dev = entry
            .properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .unwrap();

        debug!("device: {}", dev);

        if dev.contains("GoPro") {
            return Some(entry)
        }
    }

    None
}
