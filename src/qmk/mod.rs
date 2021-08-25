pub mod keycodes;

use std::fmt::{ format, Formatter, Display };
use serde::{Deserialize, Serialize};

use std::fs::{File, OpenOptions, create_dir_all};
use std::error::Error;
use std::io::{Read};
use std::path::Path;



#[derive(Debug)]
struct QmkError(String);

#[derive(Serialize, Deserialize, Clone)]
pub struct QmkLayer {
    #[serde(default)]
    pub layer: Vec<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QmkJson {
    #[serde(default)]
    pub keyboard: String,
    #[serde(default)]
    pub keymap: String,
    #[serde(default)]
    pub layers: Vec<QmkLayer>,
    #[serde(default)]
    pub layout: String,
}

/*

    Implementations

*/
impl Display for QmkError{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for QmkError{}

//
//  Defaults
//
impl Default for QmkJson {
    fn default() -> Self {
        QmkJson {
            keyboard: "Generic".to_string(),
            keymap: "None".to_string(),
            layers: Vec::new(),
            layout: "".to_string()
        }
    }
}

impl QmkJson {
    pub fn from_file<P: AsRef<Path>> (p: P) -> Result<QmkJson, Box<dyn Error>> {
        let path = p.as_ref();
        // Open the file in read-only mode with buffer.
        println!("Importing QMK json from \"{}\"", path.display());
        let mut u: QmkJson = QmkJson::default();
        if path.exists() {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            // Read the JSON contents of the file as an instance of `User`.
            u = serde_json::from_str(&contents)?;
        } else {
            return Err(Box::new(QmkError("Error because file not exits".into())));
        }
        Ok(u)
    }
}