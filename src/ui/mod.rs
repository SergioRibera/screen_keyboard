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
                ((data.styleKeyboard.keySize * margin) as u32 * data.columns) + (2.0 * data.styleKeyboard.keySize * margin) as u32
            } else {
                (data.styleKeyboard.keySize * margin) as u32 * data.rows
            };
        let height_app = (data.styleKeyboard.keySize * margin) as u32;
        // let color: CssColor = data.styleKeyboard.keyColor.parse::<CssColor>().unwrap();
        // let border_color: CssColor = data.styleKeyboard.keyBorderColor.parse::<CssColor>().unwrap();
        // let pressed_color: CssColor = data.styleKeyboard.keyPressedColor.parse::<CssColor>().unwrap();
        (MainApp {
            main_data: data.clone(),
            key_size: data.styleKeyboard.keySize,
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

    fn update(&mut self, _message: Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
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
                    row = generate_key(row, key.clone(), self.main_font, self.color, self.main_data.styleKeyboard.keyBorderRadius);
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

// pub struct FlatButton {
//     pub wid: Widget,
//     pub color: Color,
//     pub border_color: Color,
//     pub pressed_color: Color,
// }
// 
// impl FlatButton {
//     pub fn new(x: i32, y: i32,
//         w: i32, h: i32,
//         label: &str,
//         color: &str,
//         border_color: &str,
//         pressed_color: &str) -> FlatButton {
//         let _color = color.parse::<CssColor>().unwrap();
//         let _press_color = pressed_color.parse::<CssColor>().unwrap();
//         let _border_color = border_color.parse::<CssColor>().unwrap();
//         let mut x = FlatButton {
//             wid: Widget::new(x, y, w, h, None).with_label(label),
//             color: Color::from_rgb(_color.r, _color.g, _color.b),
//             border_color: Color::from_rgb(_border_color.r, _border_color.g, _border_color.b),
//             pressed_color: Color::from_rgb(_press_color.r, _press_color.g, _press_color.b),
//         };
//         x.draw();
//         x.handle();
//         x
//     }
// 
//     // Overrides the draw function
//     fn draw(&mut self) {
//         let color = self.color;
//         let border_c = self.border_color;
//         self.wid.draw(move |b| {
//             draw::draw_box(
//                 FrameType::FlatBox,
//                 b.x(),
//                 b.y(),
//                 b.width(),
//                 b.height(),
//                 color
//             );
//             draw::draw_box(
//                 FrameType::BorderFrame,
//                 b.w() - b.width() + 2, b.y(),
//                 b.width(), b.height(),
//                 border_c);
//             draw::set_draw_color(Color::White);
//             draw::set_font(Font::Courier, 24);
//             draw::draw_text2(
//                 &b.label(),
//                 b.x(),
//                 b.y(),
//                 b.width(),
//                 b.height(),
//                 Align::Center,
//             );
//         });
//     }
// 
//     // Overrides the handle function.
//     // Notice the do_callback which allows the set_callback method to work
//     fn handle(&mut self) {
//         let mut wid = self.wid.clone();
//         self.wid.handle(move |_, ev| match ev {
//             Event::Push => {
//                 wid.do_callback();
//                 true
//             }
//             _ => false,
//         });
//     }
// }
// 
// impl Deref for FlatButton {
//     type Target = Widget;
// 
//     fn deref(&self) -> &Self::Target {
//         &self.wid
//     }
// }
// 
// impl DerefMut for FlatButton {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.wid
//     }
// }
// 