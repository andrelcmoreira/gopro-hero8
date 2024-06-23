use std::env::args;
use std::io::Error;

use gopro_hero8::command::synchronous::*;

fn main() -> Result<(), Error> {
    let args = args().collect::<Vec<String>>();

    if args.len() > 1 {
        let cmd = args
            .into_iter()
            .nth(1)
            .unwrap();

        match cmd.as_str() {
            "--show-camera-info" => {
                println!("{:?}", get_factory_info()?);
                println!("{:?}", get_wifi_info()?);
                println!("{:?}", get_status_info()?)
            },
            _ => println!("command not available")
        }
    }

    Ok(())
}
