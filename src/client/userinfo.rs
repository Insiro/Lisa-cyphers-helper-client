use crate::object::charactor::CharList;
use crate::object::clan;
use crate::object::player;
use crate::util::data_serializer::option_date_se;
use crate::util::{temp, UtcTime};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use super::{ClientSave, Save};

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    players: Vec<player::PlayerBase>,
    #[serde(with = "option_date_se")]
    birth_day: Option<UtcTime>,
    clans: Vec<clan::ClanBase>,
    charactors: CharList,
    self_path: String,
}
impl Save for UserInfo {
    fn new(path: &Path) -> Self {
        match Self::load(path) {
            Ok(ok) => ok,
            Err(_) => {
                let default = Self::default();
                Self::create(&default, &path.join("user"));
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
impl ClientSave for UserInfo {
    fn default() -> Self {
        Self {
            players: Vec::new(),
            birth_day: None,
            clans: Vec::new(),
            charactors: Vec::new(),
            self_path: Self::get_default_path(),
        }
    }

    fn get_default_path() -> String {
        let mut path = temp::get_config_path();
        path.push_str("user");
        path
    }

    fn print_start_msg(&self) {
        todo!();
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
