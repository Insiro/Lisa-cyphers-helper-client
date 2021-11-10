#![allow(non_snake_case)]
use super::info;
use crate::error as lisa_error;
use crate::object;
use crate::util::temp;
use serde::{Deserialize, Serialize};
use serde_json;

pub mod Base;
pub mod Records;
mod records;

trait Player {
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str;
}

trait PlayerBuilder: Sized {
    fn new(id: String) -> Self;
    fn build(&mut self) -> Result<Box<dyn Player>, Box<dyn std::error::Error>>;
}

#[macro_export]
macro_rules! player_impl {
    ($type:ident) => {
        impl Player for $type {
            fn get_id(&self) -> &str {
                &self.playerId
            }
            fn get_name(&self) -> &str {
                &self.nickname
            }
        }
    };
}

#[derive(Deserialize, Serialize)]
pub struct Info {
    playerId: String,
    nickname: String,
    grade: u8,
    clanName: String,
    ratingPoint: Option<u8>,
    maxRatingPoint: Option<u8>,
    tierName: Option<String>,
    records: Vec<records::Records>,
}
player_impl!(Info);

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
