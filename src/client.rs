use crate::object::charactor::CharList;
use crate::object::clan;
use crate::object::player;
use chrono::{DateTime, Utc};
type UtcTime = DateTime<Utc>;
pub struct Client {
    //setting data
    pub auto_start: bool,
    pub is_nexon: bool,
    //news data
    last_notify: Option<UtcTime>,
    last_magaine: Option<UtcTime>,
    last_update: Option<UtcTime>,
    // user data
    players: Vec<player::PlayerBase>,
    birth_day: Option<UtcTime>,
    clans: Vec<clan::ClanBase>,
    charactors: CharList,
    neople_api_key: String,
}

pub fn load() -> Client {
    //TODO: load Settings from file or db

    //None Data
    Client {
        auto_start: false,
        is_nexon: true,
        last_notify: None,
        last_magaine: None,
        last_update: None,
        players: vec![],
        birth_day: None,
        clans: vec![],
        charactors: vec![],
        neople_api_key: "".to_string(),
    }
}

impl Client {
    fn get_api_key(&self) -> &str {
        &self.neople_api_key
    }
    fn open_game_page(&self) {}
    fn new(
        is_nexon: bool,
        auto_start: bool,
        players: Vec<player::PlayerBase>,
        clans: Vec<clan::ClanBase>,
        birth_day: Option<UtcTime>,
        charactors: CharList,
        last_notify: Option<UtcTime>,
        last_magaine: Option<UtcTime>,
        last_update: Option<UtcTime>,
    ) -> Client {
        Client {
            auto_start,
            is_nexon,
            players,
            birth_day,
            clans,
            charactors,
            last_notify,
            last_update,
            last_magaine,
            neople_api_key: "asdasd".to_string(),
        }
    }
    fn get_payers(&mut self) -> &mut Vec<player::PlayerBase> {
        &mut self.players
    }
    fn get_clans(&mut self) -> &mut Vec<clan::ClanBase> {
        &mut self.clans
    }
    fn set_home_type(&mut self, is_nexon: bool) {
        self.is_nexon = is_nexon
    }
    fn get_birth_day(&self) -> &Option<UtcTime> {
        &self.birth_day
    }
    fn get_charactors(&mut self) -> &CharList {
        &self.charactors
    }
}
