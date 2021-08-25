//! Display fields that can be filled with text.
use iced::{ Background, Color };
use iced::{ container::StyleSheet, container::Style };

pub struct KeyStyle {
    pub back: Background,
    pub fore: Color,
    pub border_rad: f32,
    pub border_with: f32,
    pub border_col: Color
}
impl StyleSheet for KeyStyle {
    fn style(&self) -> Style {
        Style {
            background: Some(self.back),
            text_color: Some(self.fore),
            border_radius: self.border_rad,
            border_width: self.border_with,
            border_color: self.border_col
        }
    }
}