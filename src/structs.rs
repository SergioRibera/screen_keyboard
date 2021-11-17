use std::collections::HashMap;
use indexmap::IndexMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct StyleKeyboard {
    #[serde(default)]
    pub bg_color: String,
    #[serde(default)]
    pub key_bg_color: String,
    #[serde(default)]
    pub key_color: String,
    #[serde(default)]
    pub key_pressed_color: String,
    #[serde(default)]
    pub key_border_color: String,
    #[serde(default)]
    pub key_border_radius: f32,
    #[serde(default)]
    pub key_size: f32,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Layer {
    #[serde(default)]
    pub index: u16,
    #[serde(with = "indexmap::serde_seq")]
    pub content: IndexMap<u16, String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct DataLoad {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub vendor_id: u16,
    #[serde(default)]
    pub product_id: u16,
    #[serde(default)]
    pub style_keyboard: StyleKeyboard,
    #[serde(default)]
    pub opacity: f32,
    #[serde(default)]
    pub layers: Vec<Layer>,
    #[serde(default)]
    pub split: bool,
    #[serde(default)]
    pub columns: u32,
    #[serde(default)]
    pub rows: u32
}

impl Default for Layer {
    fn default() -> Self {
        Layer {
            index: 0,
            content: IndexMap::new()
        }
    }
}
impl Default for StyleKeyboard {
    fn default() -> Self {
        StyleKeyboard {
            bg_color: "#ffffff".to_string(),
            key_bg_color: "#ccc".to_string(),
            key_color: "#000".to_string(),
            key_pressed_color: "#919191".to_string(),
            key_border_color: "#414141".to_string(),
            key_border_radius: 5.0,
            key_size: 10.0
        }
    }
}
impl Default for DataLoad {
    fn default() -> Self {
        DataLoad {
            name: "Unknown".to_string(),
            product_id: 0,
            vendor_id: 0,
            style_keyboard: StyleKeyboard::default(),
            opacity: 0.6,
            layers: Vec::new(),
            split: false,
            rows: 7,
            columns: 70
        }
    }
}