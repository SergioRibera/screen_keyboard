use std::collections::HashMap;

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
    pub keyBorderRadius: u32,
}

#[derive(Serialize, Deserialize)]
pub struct LayerSplit {
    #[serde(default)]
    pub left: HashMap<u32, String>,
    #[serde(default)]
    pub right: HashMap<u32, String>,
}

#[derive(Serialize, Deserialize)]
pub struct DataLoad {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub styleKeyboard: StyleKeyboard,
    #[serde(default)]
    pub opacity: f64,
    #[serde(default)]
    pub layers: HashMap<u32, String>,
    #[serde(default)]
    pub split: Vec<LayerSplit>
}

impl Default for StyleKeyboard {
    fn default() -> Self {
        StyleKeyboard {
            bgColor: "#ffffff".to_string(),
            keyColor: "#ccc".to_string(),
            keyPressedColor: "#919191".to_string(),
            keyBorderColor: "#414141".to_string(),
            keyBorderRadius: 5,
        }
    }
}
impl Default for LayerSplit {
    fn default() -> Self {
        LayerSplit {
            left: HashMap::new(),
            right: HashMap::new(),
        }
    }
}
impl Default for DataLoad {
    fn default() -> Self {
        DataLoad {
            name: "Unknown".to_string(),
            styleKeyboard: StyleKeyboard::default(),
            opacity: 0.6,
            layers: HashMap::new(),
            split: Vec::new(),
        }
    }
}
