use crate::error as lisa_error;
use crate::util::temp;
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Deserialize, Serialize)]
pub struct User {
    rank: u8,
    playerId: String,
    nickname: String,
    grade: u8,
    ratingPoint: u8,
    clanName: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct RankList {
    rows: Vec<User>,
}
pub fn search(id: &str) -> lisa_error::Result<User> {
    let parsed = temp::parse::ranking(id);
    let value: serde_json::Result<RankList> = serde_json::from_str(&parsed);
    return match value {
        Err(_) => Err(lisa_error::new(
            "failed to load data",
            lisa_error::Kind::DataError,
        )),
        Ok(rank) => Ok(rank.rows[0]),
    };
}
pub fn get(offset: u8, limit: u8) -> lisa_error::Result<Vec<User>> {
    //limit to 1000
    let parsed = temp::parse::ranking_list(offset, limit);
    let value: serde_json::Result<RankList> = serde_json::from_str(&parsed);
    return match value {
        Err(_) => Err(lisa_error::new(
            "failed to load data",
            lisa_error::Kind::DataError,
        )),
        Ok(rank) => Ok(rank.rows),
    };
}
pub struct charactor {}
pub struct tsj {}