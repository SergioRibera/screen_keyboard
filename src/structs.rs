use std::collections::HashMap;
use indexmap::IndexMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct StyleKeyboard {
    #[serde(default)]
    pub bgColor: String,
    #[serde(default)]
    pub keyColor: String,
    #[serde(default)]
    pub keyPressedColor: String,
    #[serde(default)]
    pub keyBorderColor: String,
    #[serde(default)]
    pub keyBorderRadius: f32,
    #[serde(default)]
    pub keySize: f32,
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
    pub vendorID: u16,
    #[serde(default)]
    pub productID: u16,
    #[serde(default)]
    pub styleKeyboard: StyleKeyboard,
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
            bgColor: "#ffffff".to_string(),
            keyColor: "#ccc".to_string(),
            keyPressedColor: "#919191".to_string(),
            keyBorderColor: "#414141".to_string(),
            keyBorderRadius: 5.0,
            keySize: 10.0
        }
    }
}
impl Default for DataLoad {
    fn default() -> Self {
        DataLoad {
            name: "Unknown".to_string(),
            productID: 0,
            vendorID: 0,
            styleKeyboard: StyleKeyboard::default(),
            opacity: 0.6,
            layers: Vec::new(),
            split: false,
            rows: 7,
            columns: 70
        }
    }
}