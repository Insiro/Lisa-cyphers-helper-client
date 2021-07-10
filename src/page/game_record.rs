use crate::object::charactor::Charactor;
use crate::object::player;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameRecord {
    player: player::ParsedPlayer,
    records: Vec<GameInfo>,
}
impl GameRecord {
    pub fn dumy() -> GameRecord {
        GameRecord {
            player: player::ParsedPlayer::dumy(),
            records: vec![],
        }
    }
    pub fn load(id: &str) -> GameRecord {
        GameRecord {
            player: player::ParsedPlayer::dumy(),
            records: vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
enum GameType {
    Normal,
    Rating,
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
}

pub fn cli(args: &mut Vec<String>, level: u8) {}
