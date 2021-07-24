use super::charactor;
use super::clan;
use crate::error as lisa_error;

use crate::util::temp::parse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

type UtcTime = Option<DateTime<Utc>>;

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

pub fn get_user_id(name: String) -> lisa_error::Result<String> {
    //TODO:change to get_from_neople(sendString)
    let rlt: Result<ParseBase, serde_json::Error> = serde_json::from_str(&parse::player_id(&name));
    match rlt {
        Ok(id) => Ok(id.nickname),
        Err(_) => Err(lisa_error::new(
            "getting Error",
            lisa_error::Kind::UserNotFoundError,
        )),
    }
}

pub fn get_user_name(id: String) -> Result<String, lisa_error::Error> {
    let name = String::from(" "); //TODO: get user name from neople API
    Ok(name)
}

pub trait Player {
    fn set_player_name(&mut self, player_name: Option<String>) -> bool;
    fn get_player_name(&self) -> &str;
    fn set_player_id(&mut self, id: Option<String>) -> lisa_error::Result<()>;
    fn get_player_id(&self) -> &Option<String>;
    fn refrash_info(&mut self) -> lisa_error::Result<()>;
}

#[derive(Serialize, Deserialize)]
pub struct PlayerBase {
    player_name: String,
    player_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ParseBase {
    playerId: String,
    nickname: String,
    grade: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ParsedPlayer {
    player_name: String,
    player_id: Option<String>,
    win_count: u8,
    lose_count: u8,
    tier: Option<String>,
    tier_point: u8,
    clan: Option<clan::ClanBase>,
}
impl ParsedPlayer {
    pub fn dumy() -> ParsedPlayer {
        ParsedPlayer {
            player_name: "".to_string(),
            player_id: None,
            win_count: 0,
            lose_count: 0,
            tier: None,
            tier_point: 0,
            clan: None,
        }
    }
    pub fn load(
        player_name: String,
        player_id: Option<String>,
        win_count: u8,
        lose_count: u8,
        tier: Option<String>,
        tier_point: u8,
        clan: Option<clan::ClanBase>,
    ) -> ParsedPlayer {
        ParsedPlayer {
            player_name,
            player_id,
            win_count,
            lose_count,
            tier,
            tier_point: 0,
            clan,
        }
    }
    pub fn new(player_id: Option<String>) -> lisa_error::Result<ParsedPlayer> {
        match player_id {
            None => return Ok(ParsedPlayer::dumy()),
            Some(id) => {
                //TODO:change to get_from_neople(snedString)
                let ps: Result<ParsedPlayer, serde_json::Error> =
                    serde_json::from_str(&parse::player_info(&id));
                match ps {
                    Err(_) => {
                        return Err(lisa_error::new(
                            "failed to parse",
                            lisa_error::Kind::UserNotFoundError,
                        ))
                    }
                    Ok(data) => return Ok(data),
                };
            }
        }
    }
}

impl Player for ParsedPlayer {
    fn set_player_name(&mut self, player_name: Option<String>) -> bool {
        match player_name {
            None => {
                //TODO: refrash Info From saved ID
                false
            }
            Some(name) => {
                self.player_name = name;
                //TODO: get Id from inputed Name
                true
            }
        }
    }
    fn set_player_id(&mut self, player_id: Option<String>) -> lisa_error::Result<()> {
        // None : Remove Player id
        match &player_id {
            None => {
                self.player_id = None;
                Ok(())
            }
            Some(id) => {
                let name = get_user_name(id.to_string());
                match name {
                    Ok(name) => {
                        self.player_id = player_id;
                        self.player_name = name;
                        Ok(())
                    }
                    Err(error) => Err(error),
                }
            }
        }
    }

    fn refrash_info(&mut self) -> lisa_error::Result<()> {
        match &self.player_id {
            None => Err(lisa_error::new(
                "not saved user info",
                lisa_error::Kind::DataError,
            )),
            Some(id) => match get_user_name(id.to_string()) {
                Ok(string) => {
                    //TODO: refrash  all feild datas from id
                    Ok(())
                }
                Err(error) => Err(error),
            },
        }
    }
    fn get_player_name(&self) -> &str {
        &self.player_name
    }

    fn get_player_id(&self) -> &Option<String> {
        &self.player_id
    }
}
