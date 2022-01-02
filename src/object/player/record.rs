#![allow(non_snake_case)]
use super::{Player, PlayerBuilder};
use crate::error as lisa_error;
use crate::object::{builder, info, neople, record, Object};
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

pub struct RecordBuilder {
    id: Option<String>,
    isNormal: bool,
    startDate: Option<UtcTime>, //api 기본값 : 현재시간
    endDate: Option<UtcTime>,   //api 기본값 : 30일 전
    limit: u8,
}
impl builder::Builder for RecordBuilder {
    type Target = Records;
    fn new() -> Self {
        Self {
            id: None,
            isNormal: false,
            startDate: None,
            endDate: None,
            limit: 100,
        }
    }
    fn build(&mut self) -> Result<Self::Target, Box<dyn std::error::Error>> {
        match &self.id {
            Some(id) => {
                let parsed = temp::parse::player_history(
                    &id,
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
            None => Err(Box::new(builder::Error::new(
                "ID not Initialized".to_string(),
            ))),
        }
    }
}
impl PlayerBuilder for RecordBuilder {
    fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }
}
impl RecordBuilder {
    pub fn set_rating_mode(&mut self, is_rating_mod: bool) {
        self.isNormal = is_rating_mod;
    }
    pub fn set_limit(&mut self, limit: u8) {
        self.limit = limit;
    }
    pub fn set_duration(&mut self, start: UtcTime, end: UtcTime) {
        self.startDate = Some(start);
        self.endDate = Some(end);
    }
}
