use std::io;

// use crate::object::charactor::Charactor;
use crate::object::player::Info;
use crate::page::matches;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameRecord {
    player: Info::Info,
    records: Vec<GameInfo>,
}
impl GameRecord {
    pub fn get_player(&mut self) -> &Info::Info {
        &self.player
    }
    pub fn get_records(&mut self) -> &Vec<GameInfo> {
        &self.records
    }
    pub fn cli(&mut self) {
        let player_name = self.player.get_name();
        let mut mat_str = String::new();
        let mut i = 0;
        for mat in self.records.iter() {
            mat_str += format!("{}  {}\t{}\n", i, mat.get_type().as_str(), mat.is_win).as_str();
            i = i + 1;
        }
        let mut select = String::new();
        let mut _selected: i8 = 0;
        loop {
            print!("{esc}c", esc = 27 as char);
            println!("player Name : {}", player_name);
            print!("matches\n{}", mat_str);
            match io::stdin().read_line(&mut select) {
                Err(_) => {
                    continue;
                }
                Ok(_) => {}
            }
            match select.trim().parse::<i8>() {
                Err(_) => {
                    println!("wrong select");
                    continue;
                }
                Ok(ok) if ok < -1 || ok > 9 => {
                    println!("wrong select");
                    continue;
                }
                Ok(ok) if ok == -1 => {
                    break;
                }
                Ok(ok) => match self.records.get(usize::from(ok.unsigned_abs())) {
                    None => {
                        println!("wrong select");
                        continue;
                    }
                    Some(st) => {
                        matches::get_match(st.get_id()).cli();
                    }
                },
            }
        }
    }
}
pub fn dumy() -> GameRecord {
    GameRecord {
        player: Info::dumy(),
        records: vec![],
    }
}
pub fn search(_id: &str) -> Option<GameRecord> {
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

use crate::command::Command;

pub fn cli(mut args: Vec<String>) -> Command {
    match args.pop() {
        None => cli_main(),
        Some(arg) => match search(&arg) {
            None => {}
            Some(_re) => {}
        },
    }
    Command::NotImpletated
}

fn cli_main() {}
pub fn help(_args: Vec<String>) {}
