use super::{ClientSave, Save};
use crate::util::temp;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Setting {
    pub is_nexon: bool,
    pub neople_api_key: String,
    pub conf_path: String,
}
impl Save for Setting {
    fn new(path: &Path) -> Self {
        match Self::load(path) {
            Ok(ok) => ok,
            Err(_) => {
                let default = Self::default();
                Self::create(&default, path);
                default
            }
        }
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_default_path();
        Self::write_file(&self, Path::new(&path))
    }

    fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let serial: String = fs::read_to_string(path)?;
        let setting = serde_json::from_str(&serial)?;
        return Ok(setting);
    }
}
impl ClientSave for Setting {
    fn print_start_msg(&self) {
        if !self.neople_api_key.eq("") {
            println!("noople API key Initailized")
        }
        if self.is_nexon {
            println!("access as nexon home");
        } else {
            println!("start as naver home mode");
        }
    }
    fn default() -> Self {
        return Self {
            is_nexon: false,
            neople_api_key: "".to_string(),
            conf_path: Self::get_default_path(),
        };
    }

    fn get_default_path() -> String {
        let mut path = temp::get_config_path();
        path.push_str("/setting");
        path
    }
}
