use crate::error as lisa_error;
use super::neople;
pub use neople::players::Info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

pub fn get_user_id(name: &str) -> lisa_error::Result<String> {
    let rlt = neople::players::Base::search(name);
    match rlt {
        Err(err) => Err(err),
        Ok(value) => Ok(value.playerId.to_string()),
    }
}

pub fn get_user_name(id: &str) -> Result<String, lisa_error::Error> {
    let rlt = neople::players::Info::get(id);
    match rlt {
        Err(err) => Err(err),
        Ok(value) => Ok(value.nickname().to_string()),
    }
}

pub trait PlayerChangerble {
    fn set_nickname(&mut self, player_name: &str) -> bool;
    fn set_player_id(&mut self, id: &str) -> lisa_error::Result<()>;
}
#[derive(Serialize, Deserialize)]
pub struct PlayerBase {
    player_name: String,
    player_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ParseBase {
    player_id: String,
    nickname: String,
    grade: u8,
}
