use super::charactor;
use super::clan;
use crate::error::{ParseResult, ParseResults, ParsingError, ParsingErrorKind};

use chrono::{DateTime, Utc};

type UtcTime = Option<DateTime<Utc>>;

pub enum Gender {
    Male,
    Female,
    Unknown,
}

pub fn get_user_id(name: String) -> ParseResults {
    let id = String::from(""); //TODO : Search user ID by neople API

    Ok(id)
}

pub fn get_user_name(id: String) -> ParseResults {
    let name = String::from(" "); //TODO: get user name from neople API
    Ok(name)
}

pub trait Player {
    fn set_player_name(&mut self, player_name: Option<String>) -> bool;
    fn get_player_name(&self) -> &str;
    fn set_player_id(&mut self, id: Option<String>) -> ParseResult<()>;
    fn get_player_id(&self) -> &Option<String>;
    fn refrash_info(&mut self) -> ParseResult<()>;
}

pub struct PlayerBase {
    player_name: String,
    player_id: Option<String>,
}

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
    pub fn new(player_id: Option<String>) -> ParsedPlayer {
        //TODO: get all data from id
        ParsedPlayer::dumy()
    }
}
pub struct SavedPlayer {
    player_info: ParsedPlayer,
    memo: Option<String>,
    gender: Gender,
    name: Option<String>,
    position: charactor::CharList,
    birth_day: UtcTime,
}
trait PlayerSavedData {
    fn set_gender(&mut self, gender: Gender);
    fn get_gender(&self) -> &Gender;
    fn set_name(&mut self);
    fn get_name(&self) -> &Option<String>;
    fn set_birthday(&mut self, birhday: UtcTime);
    fn get_birthday(&self) -> &UtcTime;
    fn get_player_info(&self) -> &ParsedPlayer;
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
    fn set_player_id(&mut self, player_id: Option<String>) -> ParseResult<()> {
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

    fn refrash_info(&mut self) -> ParseResult<()> {
        match &self.player_id {
            None => Err(ParsingError::new(
                "not saved user info".to_string(),
                ParsingErrorKind::DataError,
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

impl SavedPlayer {
    pub fn dumy() -> SavedPlayer {
        SavedPlayer {
            player_info: ParsedPlayer::dumy(),
            memo: None,
            gender: Gender::Unknown,
            name: None,
            position: vec![],
            birth_day: None,
        }
    }
    pub fn new(
        player_info: ParsedPlayer,
        memo: Option<String>,
        gender: Gender,
        name: Option<String>,
        position: charactor::CharList,
        birth_day: UtcTime,
    ) -> SavedPlayer {
        SavedPlayer {
            player_info,
            memo,
            gender,
            name,
            position,
            birth_day,
        }
    }

    pub fn get_positions(&self) -> &charactor::CharList {
        &self.position
    }
    pub fn set_birthday(&mut self, birthday: String) -> bool {
        false
    }
    pub fn set_memo(&mut self, memo: String) {
        self.memo = Some(memo);
    }
}
