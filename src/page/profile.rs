use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;

use crate::object::{charactor, player};
use crate::util::data_serializer::date_se;
use player::{Gender, Info};

type UtcTime = Option<DateTime<Utc>>;

pub fn gui() {}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    player: Info,
    memo: Option<String>,
    gender: Gender,
    name: Option<String>,
    position: charactor::CharList,
    #[serde(with = "date_se")]
    birth_day: UtcTime,
}

pub fn dumy() -> Profile {
    Profile {
        player: Info::dumy(),
        memo: None,
        gender: Gender::Unknown,
        name: None,
        position: vec![],
        birth_day: None,
    }
}
pub fn new(
    player: Info,
    memo: Option<String>,
    gender: Gender,
    name: Option<String>,
    position: charactor::CharList,
    birth_day: UtcTime,
) -> Profile {
    Profile {
        player,
        memo,
        gender,
        name,
        position,
        birth_day,
    }
}

impl Profile {
    pub fn get_player(&self) -> &Info {
        &self.player
    }
    pub fn get_positions(&self) -> &charactor::CharList {
        &self.position
    }
    pub fn set_birthday(&mut self, _birthday: String) -> bool {
        //TODO:: set birth day from string
        false
    }
    pub fn set_memo(&mut self, memo: String) {
        self.memo = Some(memo);
    }
    pub fn save(&self) -> Result<(), String> {
        //TODO: save to db 1) check have saved data 2) create or override data
        Ok(())
    }
}

pub fn search(_keyword: &str) -> Profile {
    dumy()
}

pub fn load(dir: &str) -> Result<Profile, String> {
    let path = Path::new(dir);
    if !path.exists() {
        return Err("file Error".to_string());
    }
    match fs::read_to_string(path) {
        Err(_) => Err("file Error".to_string()),
        Ok(serial) => match serde_json::from_str(serial.as_str()) {
            Err(_) => Err("data Error".to_string()),
            Ok(se) => Ok(se),
        },
    }
}

pub fn cli(mut _args: Vec<String>) {}
pub fn help(_args: Vec<String>) {}
