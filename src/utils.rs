use crate::structs::DataLoad;
use std::fs::{File, OpenOptions, create_dir_all};
use std::error::Error;
use std::io::{Read, Write};
use std::path::Path;
use dirs::config_dir;

pub fn read_user_from_file<P: AsRef<Path>>(p: P) -> Result<DataLoad, Box<dyn Error>> {
    let path = config_dir().unwrap().join("SergioRibera").join(p.as_ref());
    // Open the file in read-only mode with buffer.
    println!("Path is {}", path.display());
    let mut u: DataLoad = DataLoad::default();
    if path.exists() {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        // Read the JSON contents of the file as an instance of `User`.
        u = serde_json::from_str(&contents)?;
    } else {
        create_dir_all(path.parent().unwrap()).unwrap();
        let data_serialized = serde_json::to_string(&u).unwrap();
        let mut file: File = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.to_str().unwrap())
            .unwrap();
        file.write_all(data_serialized.as_bytes())?;
    }
    Ok(u)
}
pub fn save_into_file<P: AsRef<Path>>(p: P, data: DataLoad) {
    let path = config_dir().unwrap().join("SergioRibera").join(p.as_ref());
    create_dir_all(path.parent().unwrap()).unwrap();
    let data_serialized = serde_json::to_string(&data).unwrap();
    let mut file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.to_str().unwrap())
        .unwrap();
    file.write_all(data_serialized.as_bytes()).expect("Unable to write data");
}