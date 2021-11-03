use super::{ClientSave, Save};
use crate::util::{data_serializer::date_se, temp};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;

type UtcTime = DateTime<Utc>;
#[derive(Serialize, Deserialize)]
pub struct Subscription {
    #[serde(with = "date_se")]
    last_notify: Option<UtcTime>,
    #[serde(with = "date_se")]
    last_magazine: Option<UtcTime>,
    #[serde(with = "date_se")]
    last_update: Option<UtcTime>,
    self_path:String
}
impl Save for Subscription {
    fn new(path: &Path) -> Self {
        match Self::load(path) {
            Ok(ok) => ok,
            Err(_) => {
                let default = Self::default();
                Self::create(&default, &path.join("subscription"));
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
impl ClientSave for Subscription {
    fn default() -> Self {
        Self {
            last_notify: Some(Utc::now()),
            last_magazine: Some(Utc::now()),
            last_update: Some(Utc::now()),
            self_path:Self::get_default_path()
        }
    }

    fn get_default_path() -> String {
        let mut path = temp::get_config_path();
        path.push_str("subscription");
        path
    }

    fn print_start_msg(&self) {
        if self.last_notify.is_some() {
            println!("notify\t{}", &self.last_notify.unwrap());
        }
        if self.last_magazine.is_some() {
            println!("last_magazine\t{}", &self.last_magazine.unwrap());
        }
        if self.last_update.is_some() {
            println!("update\t{}", &self.last_update.unwrap());
        }
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
