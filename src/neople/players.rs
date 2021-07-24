use crate::error as lisa_error;

use crate::util::temp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
type UtcTime = Option<DateTime<Utc>>;

use super::info;

#[derive(Serialize, Deserialize)]
pub struct PlayerBase {
    playerId: String,
    nickname: String,
    grade: u8,
}

#[derive(Serialize, Deserialize)]
struct PlayerBaseList {
    pub rows: Vec<PlayerBase>,
}

#[derive(Serialize, Deserialize)]
struct Records {
    gameTypeId: String,
    winCount: u8,
    loseCount: u8,
    stopCount: u8,
}

#[derive(Serialize, Deserialize)]
struct MatchBase {
    date: MatchDate,
    gameTypeId: String,
    rows: Vec<MatchInfo>,
}
#[derive(Serialize, Deserialize)]
struct MatchDate {
    start: String,
    end: String,
}
#[derive(Serialize, Deserialize)]
struct MatchInfo {
    date: String,
    matchId: String,
    map: info::Map,
    playInfo: info::Play,
    position: info::Position,
}

impl super::Neople for PlayerBase {}

impl PlayerBase {
    pub fn search(_nickName: &str) -> lisa_error::Result<PlayerBase> {
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
                    false => Ok(ok.rows[0]),
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Info {
    playerId: String,
    nickname: String,
    grade: u8,
    clanName: String,
    ratingPoint: Option<u8>,
    maxRatingPoint: Option<u8>,
    tierName: Option<String>,
    records: Vec<Records>,
}
impl Info {
    pub fn get(id: &str) -> lisa_error::Result<Info> {
        let p = temp::parse::player_info(id);
        let se: serde_json::Result<Info> = serde_json::from_str(&p);
        match se {
            Err(_) => Err(lisa_error::new("parse failed", lisa_error::Kind::DataError)),
            Ok(ok) => Ok(ok),
        }
    }
}
impl Matches {
    //normal, rating
    fn parse(gameTypeId: String, startDate: UtcTime, endDate: UtcTime, limit: u8) {}
}
#[derive(Serialize, Deserialize)]
struct Matches {
    playerId: String,
    nickname: String,
    grade: u8,
    clanName: String,
    ratingPoint: Option<u8>,
    maxRatingPoint: Option<u8>,
    tierName: Option<String>,
    records: Vec<Records>,
    matches: MatchBase,
}
