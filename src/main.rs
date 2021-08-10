use std::fs::{File, OpenOptions, create_dir_all};
use std::error::Error;
use std::io::{Read, Write};
use std::path::Path;

extern crate serde;
extern crate serde_json;
extern crate css_color_parser;
#[macro_use]
extern crate lazy_static;

use css_color_parser::Color as CssColor;
use dirs::config_dir;

// use fltk::button::Button;
use fltk::enums::Mode;
// use fltk::frame::Frame;
// use fltk::group::Pack;
use fltk::{app::*, prelude::*, window::Window, enums::Event};

mod structs;
mod ui;
use structs::*;
use ui::*;


fn read_user_from_file<P: AsRef<Path>>(p: P) -> Result<DataLoad, Box<dyn Error>> {
    let path = config_dir().unwrap().join("SergioRibera").join(p.as_ref());
    // Open the file in read-only mode with buffer.
    println!("Path is {}", path.display());
    let mut u: DataLoad = DataLoad::default();
    if path.exists() {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        // Read the JSON contents of the file as an instance of `User`.
        u = serde_json::from_str(&contents)?;
    } else {
        create_dir_all(path.parent().unwrap()).unwrap();
        let data_serialized = serde_json::to_string(&u).unwrap();
        let mut file: File = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.to_str().unwrap())
            .unwrap();
        file.write_all(data_serialized.as_bytes())?;
    }
    Ok(u)
}

lazy_static! {
    static ref MAIN_DATA: DataLoad = {
        read_user_from_file("screen_keyboard.json").unwrap()
    };
}


fn main () {
    let app = App::default();
    let mut wind: Window = Window::new(100, 100, 400, 300, "Screen Keyboard");
    let col = MAIN_DATA.styleKeyboard.bgColor.parse::<CssColor>().unwrap();

    app.set_visual(Mode::empty()).unwrap();
    
    let margin = 2;

    for i in 0..7 {
        let _but = FlatButton::new((i * 60) + margin, 0 + margin,
                                  50, 50, "A",
                                  MAIN_DATA.styleKeyboard.keyColor.as_str(),
                                  MAIN_DATA.styleKeyboard.keyBorderColor.as_str(),
                                  MAIN_DATA.styleKeyboard.keyPressedColor.as_str());
        println!("i: {}", i);
    }

    // if MAIN_DATA.layers.len() == 0 && MAIN_DATA.split.len() == 0 {
    //     println!("Len of {}, {}", MAIN_DATA.layers.len(), MAIN_DATA.split.len());
    //     let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    //     pack.set_spacing(10);
    //     Frame::default().with_size(0, 40).with_label("Not Layout loaded");
    //     pack.end();
    // }

    wind.end();
    wind.show();
    wind.set_opacity(MAIN_DATA.opacity);
    wind.set_border(false); // Remove decorations
    background(col.r, col.g, col.b);
    // wind.show_with_args(&[""]);
    /*
    int MyWindow::handle(int e) {
        static int xoff = 0, yoff = 0;
        int ret = Fl_Double_Window::handle(e);
        switch ( e ) {
            // DOWNCLICK IN WINDOW CREATES CURSOR OFFSETS
            case FL_PUSH:
                xoff = x() - Fl::event_x_root();
                yoff = y() - Fl::event_y_root();
                return(1);

            case FL_DRAG:
                // DRAG THE WINDOW AROUND THE SCREEN
                position(xoff + Fl::event_x_root(), yoff + Fl::event_y_root());
                redraw();
                return(1);

            case FL_RELEASE:
                return(1);
        }
        return(ret);
    }
    */
    wind.handle(move |w, ev| {
        let mut xoff = 0;
        let mut yoff = 0;
        let xhalf = w.pixel_w() / 2;
        let yhalf = w.pixel_h() / 2;
        match ev {
            Event::Push => {
                if event_mouse_button() == fltk::app::MouseButton::Left {
                    xoff = w.x() - fltk::app::event_x_root();
                    yoff = w.y() - fltk::app::event_y_root();
                }
                true
            },
            Event::Drag => {
                if event_mouse_button() == fltk::app::MouseButton::Left {
                    w.set_pos(xoff + fltk::app::event_x_root() - xhalf, yoff + fltk::app::event_y_root() - yhalf);
                    app.redraw();
                }
                true
            },
            _ => false
        }
    });
    app.run().unwrap();
}

#[cfg(test)]
mod test;
