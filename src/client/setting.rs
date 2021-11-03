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
    conf_path: String,
    self_path: String,
}
impl Save for Setting {
    fn new(path: &Path) -> Self {
        match Self::load(path) {
            Ok(ok) => ok,
            Err(_) => {
                let default = Self::default();
                Self::create(&default, &path.join("setting"));
                default
            }
        }
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(&self.self_path);
        Self::write_file(&self, Path::new(&path))
    }

    fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let serial: String = fs::read_to_string(path)?;
        let setting = serde_json::from_str(&serial)?;
        return Ok(setting);
    }
}
impl Setting {
    pub fn set_conf_path(&mut self, new_path: &str) -> Result<(), ()> {
        let path = Path::new(new_path);
        if path.exists() {
            self.self_path = new_path.to_string();
            return Ok(());
        }
        return Err(());
    }

    pub fn get_conf_path(&self) -> &str {
        return &self.conf_path;
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
            self_path: Self::get_default_path(),
        };
    }

    fn get_default_path() -> String {
        let mut path = temp::get_config_path();
        path.push_str("setting");
        path
    }

    fn set_path(&mut self, new_path: &str) -> Result<(), ()> {
        let path = Path::new(new_path);
        if path.exists() {
            self.self_path = new_path.to_string();
            return Ok(());
        }
        return Err(());
    }
}
#[cfg(test)]
mod test {
    use super::*;
    const PATH: &str = "./test/";
    fn get_path() -> &'static Path {
        return Path::new(PATH);
    }
    #[test]
    fn gen_setting() {
        let se = Setting::new(Path::new(get_path()));
        println!("{}", se.self_path);
    }
    #[test]
    #[allow(non_snake_case)]
    fn setting_file_CUR_test() {
        let self_path = get_path().join("setting");
        let mut target = Setting::new(&self_path);
        target.self_path = self_path.to_str().unwrap().to_string();
        target.is_nexon = true;
        target.neople_api_key = "test".to_string();
        target.save().expect("failed to save test Setting");

        let setting = Setting::load(&self_path).expect("failed to load test Setting");
        assert_eq!(true, setting.is_nexon);
        assert_eq!(target.neople_api_key, setting.neople_api_key);
        assert_eq!(target.self_path, setting.self_path);
    }
}
