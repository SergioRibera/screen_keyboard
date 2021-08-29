#![allow(unused)]

pub mod styles;

use crate::utils::read_user_from_file;
use crate::structs::DataLoad;
use styles::*;
use iced::{ Background, Color, VerticalAlignment, HorizontalAlignment };
use iced::{ executor, Font, Length, Application, Clipboard, Command, Element, Column, Row, Text, Align, Container, widget::Space };

// use css_color_parser::Color as CssColor;

pub enum MainState {
}
pub struct MainApp {
    main_data: DataLoad,
    key_size: f32,
    width_app: u32,
    height_app: u32,
    margin: u16,
    main_font: Font,
    color: Color,
    border_color: Color,
    pressed_color: Color,
}
#[derive(Debug, Clone, Copy)]
pub enum Message { }

impl Application for MainApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (MainApp, Command<Message>) {
        let data: DataLoad = read_user_from_file("screen_keyboard.json").unwrap();
        let margin: f32 = 2.0;
        let width_app = if data.split {
                ((data.style_keyboard.key_size * margin) as u32 * data.columns) + (2.0 * data.style_keyboard.key_size * margin) as u32
            } else {
                (data.style_keyboard.key_size * margin) as u32 * data.rows
            };
        let height_app = (data.style_keyboard.key_size * margin) as u32;
        // let color: CssColor = data.styleKeyboard.keyColor.parse::<CssColor>().unwrap();
        // let border_color: CssColor = data.styleKeyboard.keyBorderColor.parse::<CssColor>().unwrap();
        // let pressed_color: CssColor = data.styleKeyboard.keyPressedColor.parse::<CssColor>().unwrap();
        (MainApp {
            main_data: data.clone(),
            key_size: data.style_keyboard.key_size,
            width_app: width_app,
            height_app: height_app,
            margin: margin as u16,
            main_font: Font::External {
                name: "Icons",
                bytes: include_bytes!("../../assets/DejaVuSans.ttf"),
            },
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            border_color: Color::new(0.0, 0.0, 0.0, 1.0),
            pressed_color: Color::new(0.4, 0.4, 0.4, 1.0),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("ScreenKeyboard")
    }
    fn background_color(&self) -> Color {
        Color::new(1.0, 1.0, 1.0, self.main_data.opacity)
        // Read Bg Color from json
    }

    fn update(&mut self, _message: Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        println!("Update");
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        // let size_key: u32 = WIDTH_APP / (self.main_data.layers[0].content.len() as u32) - self.margin * 2;

        let mut main_col: Column<Message> = Column::new()
                        .padding(self.margin)
                        .spacing(self.margin)
                        .align_items(Align::Center);
        let key_counts: u32 = self.main_data.columns;

        for layer in self.main_data.clone().layers {
            println!("{}", layer.clone().content.len());
            let mut row: Row<Message> = Row::new()
                        .padding(self.margin)
                        .spacing(self.margin)
                        .align_items(Align::Center);
            let mut curr_col = 0;
            for (_, key) in layer.content {
                // let _but = FlatButton::new(size_key * x + margin, size_key * y + margin,
                //                         size_key, size_key, key.as_str(),
                //                         main_data.styleKeyboard.keyColor.as_str(),
                //                         main_data.styleKeyboard.keyBorderColor.as_str(),
                //                         main_data.styleKeyboard.keyPressedColor.as_str());
                if self.main_data.split && curr_col == key_counts / 2 {
                    row = row.push(generate_space());
                    row = row.push(generate_space());
                }
                if curr_col >= key_counts {
                    // New row of keys
                    main_col= main_col.push(row);
                    row = Row::new()
                        .padding(self.margin)
                        .spacing(self.margin)
                        .align_items(Align::Center);
                    println!("{}", curr_col);
                    curr_col = 0;
                }
                if key != "KC_NO" {
                    //row = generate_key(row, key.clone(), self.main_font, self.color, self.main_data.style_keyboard.key_border_radius);
                    row = row.push(KeyWidget::new(key.clone(), self.main_font, self.color, self.main_data.style_keyboard.key_border_radius, self.main_data.style_keyboard.key_size));
                    println!("key: {:?}", key);
                } else {
                    row = row.push(generate_space());
                }
                curr_col += 1;
            }
            main_col= main_col.push(row);
        }
        main_col.into()
    }
}

fn generate_key(row: Row<Message>, key: String, font: Font, txt_color: Color, border_rad: f32) -> Row<Message> {
    let txt = Text::new(key)
                .font(font)
                .width(Length::Fill)
                .vertical_alignment(VerticalAlignment::Center)
                .horizontal_alignment(HorizontalAlignment::Center)
                .color(txt_color);
    row.push(Container::new(txt)
                .padding(10)
                .width(Length::Fill)
                .height(Length::Shrink)
                .align_x(Align::Center)
                .align_y(Align::Center)
                .style(KeyStyle {
                    back: Background::Color(Color::from_rgb(0.252, 0.252, 0.252)),
                    fore: Color::BLACK,
                    border_rad: border_rad,
                    border_with: 5.0,
                    border_col: Color::new(0.204, 0.204, 0.204, 1.0)
                }))
}

pub fn generate_space() -> Space {
    Space::new(Length::Fill, Length::Shrink)
}


use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
    layout, mouse, Hasher, Layout,
    Point, Rectangle, Size, Widget,
};

pub struct KeyWidget {
    key: String,
    font: Font,
    txt_color: Color,
    border_rad: f32,
    size: f32
}

impl KeyWidget {
    pub fn new(key: String, font: Font, txt_color: Color, border_rad: f32, size: f32) -> Self {
        Self {
            key: key,
            font: font,
            txt_color: txt_color,
            border_rad: border_rad,
            size: size
        }
    }
}

impl<Message, B> Widget<Message, Renderer<B>> for KeyWidget
where B: Backend{
    fn width(&self) -> iced::Length { Length::Fill }
    fn height(&self) -> iced::Length { Length::Shrink }
    fn layout(&self, _: &Renderer<B>, _: &iced_native::layout::Limits) -> iced_native::layout::Node {
        layout::Node::new(Size::new(self.size, self.size))
    }
    fn hash_layout(&self, state: &mut iced_native::Hasher) {
         use std::hash::Hash;

        self.size.to_bits().hash(state)
    }
    fn draw(
        &self,
        _renderer: &mut Renderer<B>,
        _defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) -> (Primitive, mouse::Interaction) {
        let mut primitives: Vec<Primitive> = Vec::new();
        primitives.push(
            Primitive::Quad {
                bounds: layout.bounds(),
                background: Background::Color(Color::BLACK),
                border_radius: self.border_rad,
                border_width: 5.0,
                border_color: Color::new(0.204, 0.204, 0.204, 1.0),
            });
        primitives.push(
            Primitive::Text {
                content: self.key.clone(),
                bounds: layout.bounds(),
                color: Color::WHITE,
                size: self.size / 2.0,
                font: self.font,
                horizontal_alignment: HorizontalAlignment::Center,
                vertical_alignment: VerticalAlignment::Center,
            }
        );
        (Primitive::Group {
            primitives: primitives
        }, mouse::Interaction::Idle)
    }
}

impl<'a, Message, B> Into<iced_native::Element<'a, Message, Renderer<B>>> for KeyWidget
where
    B: Backend,
{
    fn into(self) -> iced_native::Element<'a, Message, Renderer<B>> {
        iced_native::Element::new(self)
    }
}