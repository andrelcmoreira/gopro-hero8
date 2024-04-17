use std::collections::BTreeSet;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter,
                    Characteristic, CharPropFlags};
use btleplug::platform::{Adapter, Manager, Peripheral};
use btleplug::Error;
use uuid::Uuid;

async fn get_property(cam: &Peripheral, prop: &str, service: &str) -> String {
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

    match String::from_utf8(val) {
        Ok(val) => {
            if val.len() == 1 {
                return "NA".to_string()
            } else {
                return val
            }
        },
        Err(_) => "NA".to_string()
    }
}

pub async fn get_hw_revision(cam: &Peripheral) -> String {
    let prop = "00002a27-0000-1000-8000-00805f9b34fb";
    let service = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_property(&cam, prop, service).await;
}

pub async fn get_fw_revision(cam: &Peripheral) -> String {
    let prop = "00002a26-0000-1000-8000-00805f9b34fb";
    let service = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_property(&cam, prop, service).await;
}

pub async fn get_sw_revision(cam: &Peripheral) -> String {
    let prop = "00002a28-0000-1000-8000-00805f9b34fb";
    let service = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_property(&cam, prop, service).await;
}

pub async fn get_serial_number(cam: &Peripheral) -> String {
    let prop = "00002a25-0000-1000-8000-00805f9b34fb";
    let service = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_property(&cam, prop, service).await;
}

pub async fn get_model_number(cam: &Peripheral) -> String {
    let prop = "00002a24-0000-1000-8000-00805f9b34fb";
    let service = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_property(&cam, prop, service).await;
}

pub async fn get_manufacturer_name(cam: &Peripheral) -> String {
    let prop = "00002a29-0000-1000-8000-00805f9b34fb";
    let service = "0000180a-0000-1000-8000-00805f9b34fb";

    return get_property(&cam, prop, service).await;
}

pub async fn get_wifi_ssid(cam: &Peripheral) -> String {
    let prop = "b5f90002-aa8d-11e3-9046-0002a5d5c51b";
    let service = "b5f90001-aa8d-11e3-9046-0002a5d5c51b";

    return get_property(&cam, prop, service).await;
}

pub async fn get_wifi_password(cam: &Peripheral) -> String {
    let prop = "b5f90003-aa8d-11e3-9046-0002a5d5c51b";
    let service = "b5f90001-aa8d-11e3-9046-0002a5d5c51b";

    return get_property(&cam, prop, service).await;
}

// TODO: return u8
pub async fn get_battery_level(cam: &Peripheral) -> String {
    let prop = "00002a19-0000-1000-8000-00805f9b34fb";
    let service = "0000180f-0000-1000-8000-00805f9b34fb";

    return get_property(&cam, prop, service).await;
}

// TODO: return i8
pub async fn get_tx_power_level(cam: &Peripheral) -> String {
    let prop = "00002a07-0000-1000-8000-00805f9b34fb";
    let service = "00001804-0000-1000-8000-00805f9b34fb";

    return get_property(&cam, prop, service).await;
}

pub async fn get_characteristic_cfg(cam: &Peripheral) -> String {
    let prop = "b5f90005-aa8d-11e3-9046-0002a5d5c51b";
    let service = "b5f90001-aa8d-11e3-9046-0002a5d5c51b";

    return get_property(&cam, prop, service).await;
}

// TODO: discover what this value represents
pub async fn get_unknown_field(cam: &Peripheral) -> String {
    let prop = "b5f90006-aa8d-11e3-9046-0002a5d5c51b";
    let service = "b5f90001-aa8d-11e3-9046-0002a5d5c51b";

    return get_property(&cam, prop, service).await;
}

pub async fn get_bt_adapter() -> Result<Adapter, Error> {
    let mgr = Manager::new().await?;
    let adapter = mgr
        .adapters()
        .await?
        .into_iter()
        .nth(0)
        .unwrap();

    Ok(adapter)
}

pub async fn connect_to_cam(adapter: &Adapter) -> Result<Peripheral, Error> {
    adapter.start_scan(ScanFilter::default()).await?;

    let cam = find_camera(&adapter)
        .await
        .unwrap();
    if ! cam.is_connected().await? {
        println!("trying to connect...");
        cam.connect().await?
    }

    println!("is connected? {}", cam.is_connected().await?);

    cam.discover_services().await?;

    Ok(cam)
}

pub async fn show_camera_info() -> Result<(), Error> {
    //for s in cam.services() {
    //    println!(
    //        "Service UUID {}, primary: {}",
    //        s.uuid, s.primary
    //    );
    //    for characteristic in s.characteristics {
    //        println!("{:?}", characteristic);
    //    }
    //}

    //cam.disconnect().await?;
    Ok(())
}

async fn find_camera(adapter: &Adapter) -> Option<Peripheral> {
    let devices = adapter
        .peripherals()
        .await
        .unwrap();

    for dev in devices {
        let is_gopro = dev
            .properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .unwrap()
            .contains("GoPro");

        if is_gopro {
            return Some(dev)
        }
    }

    None
}
