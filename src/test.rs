use crate::structs::DataLoad;

use super::*;

#[test]
fn test_incomplete_load() {
    let text = r#"
    {
        "name": "Test Name"
    }
    "#;
    let data: DataLoad = serde_json::from_str(text).unwrap();
    assert_eq!(data.name, "Test Name");
    assert_eq!(data.style_keyboard.bg_color, "#ffffff");
}

#[test]
fn test_incomplete_complex_load() {
    let text = r#"
    {
        "name": "Test Name",
        "layers": [
            {
                "index": 0,
                "content": [
                    [0, "q"]
                ]
            }
        ]
    }
    "#;
    let data: DataLoad = serde_json::from_str(text).unwrap();
    assert_eq!(data.name, "Test Name");
    assert_eq!(data.product_id, 0);
    assert_eq!(data.opacity, 0.6);
    assert_eq!(data.layers.len(), 1);
    assert_eq!(data.layers.get(0).unwrap().content.len(), 1);
    // assert_eq!(data.layers.get(0), "a");
}
