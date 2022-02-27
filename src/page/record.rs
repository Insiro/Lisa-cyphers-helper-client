use crate::object::player;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Record {
    player: player::Info,
    records: Vec<GameInfo>,
}
impl Record {
    pub fn get_player(&mut self) -> &player::Info {
        &self.player
    }
    pub fn get_records(&mut self) -> &Vec<GameInfo> {
        &self.records
    }
}

pub fn dumy() -> Record {
    Record {
        player: player::Info::dumy(),
        records: vec![],
    }
}
pub fn search(_id: &str) -> Option<Record> {
    //TODO: search by api
    Some(dumy())
}

#[derive(Serialize, Deserialize)]
pub enum GameType {
    Normal,
    Rating,
}

impl GameType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GameType::Normal => "Normal",
            GameType::Rating => "Rating",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    match_id: String,
    game_type: GameType,
    is_win: bool,
}
impl GameInfo {
    pub fn dumy() -> GameInfo {
        GameInfo {
            match_id: "".to_string(),
            game_type: GameType::Normal,
            is_win: true,
        }
    }
    pub fn get_type(&self) -> &GameType {
        &self.game_type
    }
    pub fn win(self) -> bool {
        self.is_win
    }
    pub fn get_id(&self) -> &String {
        &self.match_id
    }
}
