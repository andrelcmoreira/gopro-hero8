use std::env::args;
use std::io::Error;

use log::{debug, error, info};

use gopro_hero8::api::synchronous as sync;

fn main() -> Result<(), Error> {
    let args = args().collect::<Vec<String>>();

    env_logger::init();

    if args.len() > 1 {
        let cmd = args
            .into_iter()
            .nth(1)
            .unwrap();

        debug!("command: {}", cmd);

        match cmd.as_str() {
            "--show-camera-info" => {
                match sync::get_factory_info() {
                    Ok(data) => info!("{data:?}"),
                    Err(err) => error!("{}", err.to_string())
                };
                match sync::get_wifi_info() {
                    Ok(data) => info!("{data:?}"),
                    Err(err) => error!("{}", err.to_string())
                };
                match sync::get_status_info() {
                    Ok(data) => info!("{data:?}"),
                    Err(err) => error!("{}", err.to_string())
                };
            },
            _ => error!("command not available")
        }
    }

    Ok(())
}
