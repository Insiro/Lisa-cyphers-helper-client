use crate::object::charactor::CharList;
use crate::object::clan;
use crate::object::player;
use crate::util::data_serializer::date_se;
use crate::util::temp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

type UtcTime = DateTime<Utc>;

use super::{ClientSave, Save};

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    players: Vec<player::PlayerBase>,
    #[serde(with = "date_se")]
    birth_day: Option<UtcTime>,
    clans: Vec<clan::ClanBase>,
    charactors: CharList,
}
impl Save for UserInfo {
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
impl ClientSave for UserInfo {
    fn default() -> Self {
        Self {
            players: Vec::new(),
            birth_day: None,
            clans: Vec::new(),
            charactors: Vec::new(),
        }
    }

    fn get_default_path() -> String {
        let mut path = temp::get_config_path();
        path.push_str("/user");
        path
    }

    fn print_start_msg(&self) {
        todo!();
    }
}
