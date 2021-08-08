#![allow(dead_code, non_snake_case)]
use crate::error as lisa_error;

use crate::util::temp;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json;
type UtcTime = Option<DateTime<Utc>>;
use super::records;
// use getset::Getters;
use super::info;

#[derive(Deserialize)]
pub struct Matches {
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
impl Matches {
    //normal, rating
    fn parse(
        id: &str,
        isNormal: bool,
        startDate: UtcTime,
        endDate: UtcTime,
        limit: u8,
    ) -> lisa_error::Result<Matches> {
        let parsed = temp::parse::player_history(id, isNormal, startDate, endDate, limit);
        let se: serde_json::Result<Matches> = serde_json::from_str(&parsed);
        match se {
            Err(_) => Err(lisa_error::new("parse failed", lisa_error::Kind::DataError)),
            Ok(ok) => Ok(ok),
        }
    }
}

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

impl super::super::Neople for Matches {}
