use std::env::args;

use gp8ctl::bluetooth::*;
use gp8ctl::wifi::*;

fn main() {
    let args = args().collect::<Vec<String>>();

    if args.len() > 1 {
        let cmd = args
            .into_iter()
            .nth(1)
            .unwrap();

        match cmd.as_str() {
            "--show-camera-info" => show_camera_info(),
            "--list-media" => list_media(),
            _ => println!("command not available")
        }
    }
}
