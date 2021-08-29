
extern crate serde;
extern crate serde_json;
extern crate css_color_parser;

use iced::Settings;
use iced::Application;

mod ui;
mod consts;
mod utils;
mod structs;
mod qmk;

use std::thread;
use consts::*;
use ui::MainApp;


fn main () -> iced::Result {
    // Devices RAW HID

    thread::spawn(|| {
        let api = hidapi::HidApi::new().unwrap();
        // for device in api.devices() {
        //     println!("{:?}", device);
        // }

        println!("Opening...");
        // Connect to device using its VID and PID (Ruyi controller)
        let (vid, pid) = (0x4653, 0x0001);
        let device = api.open(vid, pid).unwrap();
        let manufacturer = device.get_manufacturer_string().unwrap();
        let product = device.get_product_string().unwrap();
        println!("Product {:?} Device {:?}", manufacturer, product);

        loop {
            println!("Reading...");
            // Read data from device
            let mut buf = [0u8; 8];
            let res = device.read(&mut buf[..]).unwrap();
            println!("Read: {:?}", &buf[..res]);
        }
    });

    // Application GUI

    let sets = Settings {
        window: iced::window::Settings {
            size: (WIDTH_APP, HEIGHT_APP),
            always_on_top: true,
            decorations: false,
            transparent: true,
            resizable: false,
            ..iced::window::Settings::default()
        },
        ..Default::default()
    };
    MainApp::run(sets)
}

#[cfg(test)]
mod test;
