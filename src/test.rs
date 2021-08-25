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
    assert_eq!(data.styleKeyboard.bgColor, "#ffffff");
}

#[test]
fn test_incomplete_complex_load() {
    let text = r#"
    {
        "name": "Test Name",
        "layers": [
            "0": [ 123: "a" ]
        ]
    }
    "#;
    let data: DataLoad = serde_json::from_str(text).unwrap();
    assert_eq!(data.name, "Test Name");
    assert_eq!(data.layers.len(), 1);
    println!("{:?}", data.layers.get(0).unwrap().index);
    // assert_eq!(data.layers.get(0), "a");
}
