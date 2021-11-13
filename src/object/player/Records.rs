#![allow(non_snake_case)]
use super::super::info;
use super::super::record;
use super::{Player, PlayerBuilder};
use crate::error as lisa_error;
use crate::object::{neople, Object};
use crate::player_impl_neople;
use crate::util::{temp, UtcTime};
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
pub struct Records {
    playerId: String,
    nickname: String,
    pub grade: u8,
    clanName: String,
    pub ratingPoint: Option<u8>,
    pub maxRatingPoint: Option<u8>,
    tierName: Option<String>,
    records: Vec<record::Record>,
    matches: MatchBase,
}
impl neople::Object for Records {}
impl Object for Records {}
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

player_impl_neople!(Records);

struct RecordBuilder {
    id: String,
    isNormal: bool,
    startDate: Option<UtcTime>, //api 기본값 : 현재시간
    endDate: Option<UtcTime>,   //api 기본값 : 30일 전
    limit: u8,
}

impl PlayerBuilder for RecordBuilder {
    type Player = Records;
    fn new(id: String) -> Result<Self, Box<(dyn std::error::Error)>> {
        Ok(Self {
            id,
            isNormal: false,
            startDate: None,
            endDate: None,
            limit: 100,
        })
    }
    fn build(&self) -> Result<Self::Player, Box<dyn std::error::Error>> {
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
            Ok(ok) => Ok(ok),
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
