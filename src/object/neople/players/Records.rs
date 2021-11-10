#![allow(dead_code, non_snake_case)]
use super::{info, PlayerBuilder};
use super::{records, Player};
use crate::error as lisa_error;
use crate::player_impl;
use crate::util::{temp, UtcTime};
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
pub struct Records {
    playerId: String,
    nickname: String,
    grade: u8,
    clanName: String,
    ratingPoint: Option<u8>,
    maxRatingPoint: Option<u8>,
    tierName: Option<String>,
    records: Vec<records::Records>,
    matches: MatchBase,
}
impl Records {}

#[derive(Deserialize)]
pub struct MatchInfo {
    date: String,
    matchId: String,
    map: info::Map,
    playInfo: info::Play,
    position: info::Position,
}
#[derive(Deserialize)]
pub struct MatchBase {
    pub date: MatchDate,
    pub gameTypeId: String,
    pub rows: Vec<MatchInfo>,
}
#[derive(Deserialize)]
pub struct MatchDate {
    start: String,
    end: String,
}

player_impl!(Records);

struct RecordBuilder {
    id: String,
    isNormal: bool,
    startDate: Option<UtcTime>, //현재시간
    endDate: Option<UtcTime>,   //30분 전
    limit: u8,
}

impl PlayerBuilder for RecordBuilder {
    fn new(id: String) -> Self {
        Self {
            id,
            isNormal: false,
            startDate: None,
            endDate: None,
            limit: 100,
        }
    }
    fn build(&mut self) -> Result<Box<dyn Player>, Box<dyn std::error::Error>> {
        let parsed = temp::parse::player_history(
            &self.id,
            self.isNormal,
            &self.startDate,
            &self.endDate,
            self.limit,
        );
        let se: serde_json::Result<Records> = serde_json::from_str(&parsed);
        match se {
            Err(_) => Err(Box::new(lisa_error::new(
                "parse failed",
                lisa_error::Kind::DataError,
            ))),
            Ok(ok) => Ok(Box::new(ok)),
        }
    }
}
impl RecordBuilder {
    fn set_rating_mode(&mut self, is_rating_mod: bool) {
        self.isNormal = is_rating_mod;
    }
    fn set_limit(&mut self, limit: u8) {
        self.limit = limit;
    }
    fn set_duration(&mut self, start: UtcTime, end: UtcTime) {
        self.startDate = Some(start);
        self.endDate = Some(end);
    }
}
