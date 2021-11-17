#![allow(unused)]

pub mod styles;

use crate::utils::read_user_from_file;
use crate::structs::{ DataLoad, StyleKeyboard };
use styles::*;
use iced::{ Background, Color, VerticalAlignment, HorizontalAlignment };
use iced::{ executor, Font, Length, Application, Clipboard, Command, Element, Column, Row, Text, Align, Container, widget::Space };

use css_color_parser::Color as CssColor;

pub enum MainState {
}
pub struct MainApp {
    main_data: DataLoad,
    key_size: f32,
    width_app: u32,
    height_app: u32,
    margin: u16,
    main_font: Font,
    background: Color,
    style: KeyStyle
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    KeyPress,
    KeyRelease
}

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
        let bg: CssColor = data.style_keyboard.bg_color.parse::<CssColor>().unwrap();
        let key_bg: CssColor = data.style_keyboard.key_bg_color.parse::<CssColor>().unwrap();
        let color: CssColor = data.style_keyboard.key_color.parse::<CssColor>().unwrap();
        let border_color: CssColor = data.style_keyboard.key_border_color.parse::<CssColor>().unwrap();
        let pressed_color: CssColor = data.style_keyboard.key_pressed_color.parse::<CssColor>().unwrap();
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
            background: Color::new((bg.r / 255) as f32, (bg.g / 255) as f32, (bg.b / 255) as f32, data.opacity),
            style: KeyStyle {
                back: Background::from(Color::new((key_bg.r / 255) as f32, (key_bg.g / 255) as f32, (key_bg.b / 255) as f32, key_bg.a / 255.0)),
                fore: Color::new((color.r / 255) as f32, (color.g / 255) as f32, (color.b / 255) as f32, color.a / 255.0),
                border_rad: data.style_keyboard.key_border_radius,
                border_width: 2.0,
                border_col: Color::new((border_color.r / 255) as f32, (border_color.g / 255) as f32, (border_color.b / 255) as f32, border_color.a / 255.0),
                press_col: Color::new((pressed_color.r / 255) as f32, (pressed_color.g / 255) as f32, (pressed_color.b / 255) as f32, pressed_color.a / 255.0),
            },
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("ScreenKeyboard")
    }
    fn background_color(&self) -> Color {
        self.background
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
                    // row = generate_key(row, key.clone(), self.main_font, self.style);
                    row = row.push(KeyWidget::new(key.clone(), self.main_font, self.style, self.key_size));
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

fn generate_key(row: Row<Message>, key: String, font: Font, style: KeyStyle) -> Row<Message> {
    let txt = Text::new(key)
                .font(font)
                .width(Length::Fill)
                .vertical_alignment(VerticalAlignment::Center)
                .horizontal_alignment(HorizontalAlignment::Center)
                .color(Color::BLACK);
    row.push(Container::new(txt)
                .padding(10)
                .width(Length::Fill)
                .height(Length::Shrink)
                .align_x(Align::Center)
                .align_y(Align::Center)
                .style(style))
}

pub fn generate_space() -> Space {
    Space::new(Length::Fill, Length::Shrink)
}


use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
    event::Status, event::Event,
    keyboard::KeyCode, keyboard::Modifiers,
    layout, mouse, Hasher, Layout,
    Point, Rectangle, Size, Widget,
};

pub struct KeyWidget {
    key: String,
    font: Font,
    style: KeyStyle,
    size: f32
}

impl KeyWidget {
    pub fn new(key: String, font: Font, style: KeyStyle, size: f32) -> Self {
        Self {
            key: key,
            font: font,
            style: style,
            size: size
        }
    }
}

impl<Message, B> Widget<Message, Renderer<B>> for KeyWidget
where B: Backend {
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
                background: self.style.back,
                border_radius: self.style.border_rad,
                border_width: self.style.border_width,
                border_color: self.style.border_col,
            });
        primitives.push(
            Primitive::Text {
                content: self.key.clone(),
                bounds: layout.bounds(),
                color: self.style.fore,
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
    fn on_event(
        &mut self, event: Event,
        _layout: Layout<'_>, _cursor_position: Point,
        _renderer: &Renderer<B>, _clipboard: &mut dyn iced_native::Clipboard,
        messages: &mut Vec<Message>,) -> Status {
        let key: char = self.key.as_str().chars().next().unwrap();
        match event {
            Event::Keyboard(iced::keyboard::Event::CharacterReceived(key_char)) if !key_char.is_control() => {
                if key == key_char {
                    messages.push(Some("KeyPress".clone()));
                }
                Status::Captured
            },
            _ => {
                Status::Ignored
            }
        }
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