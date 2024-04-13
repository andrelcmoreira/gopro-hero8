use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, Characteristic, CharPropFlags};
use btleplug::platform::{Adapter, Manager, Peripheral};
use btleplug::Error;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;
use std::collections::BTreeSet;

pub async fn show_camera_info() -> Result<(), Error> {
    let mgr = Manager::new().await?;
    let adapter = mgr
        .adapters()
        .await?
        .into_iter()
        .nth(0)
        .unwrap();

    adapter.start_scan(ScanFilter::default()).await?;

    time::sleep(Duration::from_secs(2)).await;

    let cam = find_camera(&adapter).await.unwrap();

    println!("is connected? {}", cam.is_connected().await?);

    if ! cam.is_connected().await? {
        println!("trying to connect...");
        cam.connect().await?
    }

    println!("is connected? {}", cam.is_connected().await?);

    cam.discover_services().await?;
    //for s in cam.services() {
    //    println!(
    //        "Service UUID {}, primary: {}",
    //        s.uuid, s.primary
    //    );
    //    for characteristic in s.characteristics {
    //        println!("{:?}", characteristic);
    //    }
    //}

    let ch = Characteristic {
        uuid: Uuid::parse_str("b5f90006-aa8d-11e3-9046-0002a5d5c51b")?,
        service_uuid: Uuid::parse_str("b5f90001-aa8d-11e3-9046-0002a5d5c51b")?,
        properties: CharPropFlags::READ,
        descriptors: BTreeSet::new()
    };

    println!("waiting for reply...");
    let ret = cam.read(&ch).await?;
    println!("reply received!");

    println!("raw ret = {:?}", ret);
    println!("ret = {}", String::from_utf8(ret).unwrap());

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
