#![allow(dead_code, non_snake_case)]
use crate::error as lisa_error;
use crate::object;
use crate::util::temp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
type UtcTime = Option<DateTime<Utc>>;
use super::info;
use getset::Getters;

mod Matches;
mod records;

#[derive(Deserialize, Clone)]
pub struct Base {
    pub playerId: String,
    pub nickname: String,
    pub grade: u8,
}

impl Base {
    pub fn search(_nickName: &str) -> lisa_error::Result<Base> {
        let p = temp::parse::player_id(_nickName);
        let obj: serde_json::Result<PlayerBaseList> = serde_json::from_str(p.as_str());
        match obj {
            Err(_) => Err(lisa_error::new("failed parse", lisa_error::Kind::DataError)),
            Ok(ok) => {
                let row = ok.rows;
                match row.is_empty() {
                    true => Err(lisa_error::new(
                        "Not Found",
                        lisa_error::Kind::UserNotFoundError,
                    )),
                    false => Ok(row[0].clone()),
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, Getters)]
pub struct Info {
    #[getset(get = "pub")]
    playerId: String,
    #[getset(get = "pub")]
    nickname: String,
    #[getset(get = "pub")]
    grade: u8,
    #[getset(get = "pub")]
    clanName: String,
    #[getset(get = "pub")]
    ratingPoint: Option<u8>,
    #[getset(get = "pub")]
    maxRatingPoint: Option<u8>,
    #[getset(get = "pub")]
    tierName: Option<String>,
    #[getset(get = "pub")]
    records: Vec<records::Records>,
}

impl super::Neople for Base {}
impl super::Neople for Info {}

impl Info {
    pub fn get(id: &str) -> lisa_error::Result<Info> {
        let parsed = temp::parse::player_info(id);
        let se: serde_json::Result<Info> = serde_json::from_str(&parsed);
        match se {
            Err(_) => Err(lisa_error::new("parse failed", lisa_error::Kind::DataError)),
            Ok(ok) => Ok(ok),
        }
    }
    fn update(&mut self, from: Info) {
        self.nickname = from.nickname;
        self.grade = from.grade;
        self.clanName = from.clanName;
        self.maxRatingPoint = from.maxRatingPoint;
        self.ratingPoint = from.ratingPoint;
        self.tierName = from.tierName;
        self.records = from.records;
    }
    pub fn dumy() -> Info {
        Info {
            playerId: "dumy".to_string(),
            nickname: "dumy".to_string(),
            grade: 0,
            clanName: "dumy".to_string(),
            ratingPoint: None,
            maxRatingPoint: None,
            tierName: None,
            records: vec![records::dumy()],
        }
    }
}

impl object::Objects for Info {
    fn refrash(&mut self) -> lisa_error::Result<()> {
        match Info::get(&self.playerId) {
            Err(err) => Err(err),
            Ok(value) => {
                self.update(value);
                Ok(())
            }
        }
    }
}

#[derive(Deserialize)]
struct PlayerBaseList {
    pub rows: Vec<Base>,
}
