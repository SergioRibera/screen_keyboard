
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

use consts::*;
use ui::MainApp;


fn main () -> iced::Result {
    // let col = main_data.styleKeyboard.bgColor.parse::<CssColor>().unwrap();

    // let mut layerBase: Layer = Layer::default();
    // layerBase.content.insert(1, String::from("⨮"));
    // layerBase.content.insert(2, String::from("q"));
    // layerBase.content.insert(3, String::from("w"));
    // layerBase.content.insert(4, String::from("e"));
    // layerBase.content.insert(5, String::from("r"));
    // layerBase.content.insert(6, String::from("t"));
    // layerBase.content.insert(7, String::from("y"));
    // layerBase.content.insert(8, String::from("u"));
    // layerBase.content.insert(9, String::from("i"));
    // layerBase.content.insert(10, String::from("o"));
    // layerBase.content.insert(11, String::from("p"));
    // layerBase.content.insert(12, String::from("➜"));
    // let mut layer1: Layer = Layer::default();
    // layer1.content.insert(1, String::from("⇆"));
    // layer1.content.insert(2, String::from("a"));
    // layer1.content.insert(3, String::from("s"));
    // layer1.content.insert(4, String::from("d"));
    // layer1.content.insert(5, String::from("f"));
    // layer1.content.insert(6, String::from("g"));
    // layer1.content.insert(7, String::from("h"));
    // layer1.content.insert(8, String::from("j"));
    // layer1.content.insert(9, String::from("k"));
    // layer1.content.insert(10, String::from("l"));
    // layer1.content.insert(11, String::from(";"));
    // layer1.content.insert(12, String::from("\""));
    // main_data.layers.insert(0, layerBase.clone());
    // main_data.layers.insert(1, layer1.clone());
    // save_into_file("screen_keyboard.json", main_data.clone());



    let mut sets = Settings {
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
    // if MAIN_DATA.layers.len() == 0 && MAIN_DATA.split.len() == 0 {
    //     println!("Len of {}, {}", MAIN_DATA.layers.len(), MAIN_DATA.split.len());
    //     let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    //     pack.set_spacing(10);
    //     Frame::default().with_size(0, 40).with_label("Not Layout loaded");
    //     pack.end();
    // }

    // wind.set_opacity(main_data.opacity);
    // background(col.r, col.g, col.b);

    // wind.handle(move |w, ev| {
    //     let mut xoff = 0;
    //     let mut yoff = 0;
    //     let xhalf = w.pixel_w() / 2;
    //     let yhalf = w.pixel_h() / 2;
    //     match ev {
    //         Event::Push => {
    //             if event_mouse_button() == fltk::app::MouseButton::Left {
    //                 xoff = w.x() - fltk::app::event_x_root();
    //                 yoff = w.y() - fltk::app::event_y_root();
    //             } else if event_mouse_button() == MouseButton::Right {
    //                 let mut adjust_window = Window::default()
    //                     .with_label("Settings")
    //                     .with_size(200, 400)
    //                     .center_screen();
    //                 adjust_window.set_border(true);
    //                 adjust_window.make_modal(true);
    //                 adjust_window.end();
    //                 adjust_window.show();
    //             }
    //             true
    //         },
    //         Event::Drag => {
    //             if event_mouse_button() == fltk::app::MouseButton::Left {
    //                 w.set_pos(xoff + fltk::app::event_x_root() - xhalf, yoff + fltk::app::event_y_root() - yhalf);
    //                 app.redraw();
    //             }
    //             true
    //         },
    //         _ => false
    //     }
    // });
    // app.run().unwrap();
}

#[cfg(test)]
mod test;
