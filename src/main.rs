use std::env::args;
use std::io::Error;

use log::{info, error};

use gopro_hero8::command::synchronous::*;

fn main() -> Result<(), Error> {
    let args = args().collect::<Vec<String>>();

    env_logger::init();

    if args.len() > 1 {
        let cmd = args
            .into_iter()
            .nth(1)
            .unwrap();

        match cmd.as_str() {
            "--show-camera-info" => {
                info!("{:?}", get_factory_info()?);
                info!("{:?}", get_wifi_info()?);
                info!("{:?}", get_status_info()?)
            },
            _ => error!("command not available")
        }
    }

    Ok(())
}
