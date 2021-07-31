use fltk::{
    draw, enums::*, prelude::*,
    widget::Widget,
};
use std::ops::{Deref, DerefMut};

extern crate css_color_parser;
use css_color_parser::Color as CssColor;

pub struct FlatButton {
    pub wid: Widget,
    pub color: Color,
    pub pressedColor: Color,
}

impl FlatButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &'static str, color: &'static str, pressedColor: &'static str) -> FlatButton {
        let _color = color.parse::<CssColor>().unwrap();
        let _press_color = pressedColor.parse::<CssColor>().unwrap();
        let mut x = FlatButton {
            wid: Widget::new(x, y, w, h, label),
            color: Color::from_rgb(_color.r, _color.g, _color.b),
            pressedColor: Color::from_rgb(_press_color.r, _press_color.g, _press_color.b),
        };
        x.draw();
        x.handle();
        x
    }

    // Overrides the draw function
    fn draw(&mut self) {
        let color = self.color;
        self.wid.draw(move |b| {
            draw::draw_box(
                FrameType::FlatBox,
                b.x(),
                b.y(),
                b.width(),
                b.height(),
                color
            );
            draw::draw_box(
                FrameType::BorderBox,
                b.w(), b.y(),
                b.width(), b.height(),
                Color::Black);
            draw::set_draw_color(Color::White);
            draw::set_font(Font::Courier, 24);
            draw::draw_text2(
                &b.label(),
                b.x(),
                b.y(),
                b.width(),
                b.height(),
                Align::Center,
            );
        });
    }

    // Overrides the handle function.
    // Notice the do_callback which allows the set_callback method to work
    fn handle(&mut self) {
        let mut wid = self.wid.clone();
        self.wid.handle(move |_, ev| match ev {
            Event::Push => {
                wid.do_callback();
                true
            }
            _ => false,
        });
    }
}

impl Deref for FlatButton {
    type Target = Widget;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for FlatButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
