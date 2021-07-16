use crate::object::charactor::Charactor;
use crate::object::player;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameRecord {
    player: player::ParsedPlayer,
    records: Vec<GameInfo>,
}
impl GameRecord {
    pub fn get_player(&mut self) -> &player::ParsedPlayer {
        &self.player
    }
    pub fn get_records(&self) -> &Vec<GameInfo> {
        &self.records
    }
}
pub fn dumy() -> GameRecord {
    GameRecord {
        player: player::ParsedPlayer::dumy(),
        records: vec![],
    }
}
pub fn search(id: &str) -> Option<GameRecord> {
    //TODO: search by api
    Some(dumy())
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

pub fn cli(args: &mut Vec<&str>, position: usize) {
    match args.get(position) {
        None => cli_main(),
        Some(arg) => match search(arg) {
            None => {}
            Some(re) => {}
        },
    }
}

fn cli_main() {}
fn cli_searched(){
    
}