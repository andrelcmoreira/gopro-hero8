use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;

pub async fn show_camera_info() -> Result<(), Box<dyn Error>> {
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
        cam.connect().await?
    }

    println!("is connected? {}", cam.is_connected().await?);

    //cam.discover_services().await?;

    //for s in cam.services() {
    //    println!("{}", s.uuid)
    //}

    //println!("TODO: camera info");

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
