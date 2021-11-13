use crate::object::{charactor, player};
use crate::util::data_serializer::option_date_se;
use crate::util::UtcTime;
use player::Gender;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;

pub fn gui() {}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    player: player::Info,
    memo: Option<String>,
    gender: Gender,
    name: Option<String>,
    position: charactor::CharList,
    #[serde(with = "option_date_se")]
    birth_day: Option<UtcTime>,
}

pub fn dumy() -> Profile {
    Profile {
        player: player::Info::dumy(),
        memo: None,
        gender: Gender::Unknown,
        name: None,
        position: vec![],
        birth_day: None,
    }
}
pub fn new(
    player: player::Info,
    memo: Option<String>,
    gender: Gender,
    name: Option<String>,
    position: charactor::CharList,
    birth_day: Option<UtcTime>,
) -> Profile {
    Profile {
        player,
        memo,
        gender,
        name,
        position,
        birth_day,
    }
}

impl Profile {
    pub fn get_player(&self) -> &player::Info {
        &self.player
    }
    pub fn get_positions(&self) -> &charactor::CharList {
        &self.position
    }
    pub fn set_birthday(&mut self, _birthday: String) -> bool {
        //TODO:: set birth day from string
        false
    }
    pub fn set_memo(&mut self, memo: String) {
        self.memo = Some(memo);
    }
    pub fn save(&self) -> Result<(), String> {
        //TODO: save to db 1) check have saved data 2) create or override data
        Ok(())
    }
    pub fn search(_keyword: &str) -> Option<Profile> {
        None
    }
}

pub fn load(dir: &str) -> Result<Profile, String> {
    let path = Path::new(dir);
    if !path.exists() {
        return Err("file Error".to_string());
    }
    match fs::read_to_string(path) {
        Err(_) => Err("file Error".to_string()),
        Ok(serial) => match serde_json::from_str(serial.as_str()) {
            Err(_) => Err("data Error".to_string()),
            Ok(se) => Ok(se),
        },
    }
}
use crate::command::Command;

#[derive(PartialEq)]
enum Commands {
    Get,
    Remove,
    Others,
    Help,
    Load,
}
impl Commands {
    fn from_str(com: &str) -> Commands {
        match com.trim() {
            "get" => Commands::Get,
            "help" => Commands::Help,
            "load" => Commands::Load,
            _ => Commands::Others,
        }
    }
}
pub fn cli(mut args: Vec<String>) -> Command {
    match args.pop() {
        None => cli_main(),
        Some(command) => {
            let com = Commands::from_str(&command);
            match com {
                Commands::Others => return Command::from_str(&command),
                Commands::Help => {
                    help(args);
                    Command::Help
                }
                Commands::Get => match args.pop() {
                    Some(key) => match Profile::search(&key) {
                        None => {
                            println!("not founed on saved profile");
                            Command::Profile
                        }
                        Some(profile) => Command::Some,
                    },
                    None => Command::Profile,
                },
                Commands::Remove | Commands::Load => Command::NotImpletated,
            }
        }
    }
}
pub fn help(_args: Vec<String>) {}
fn cli_main() -> Command {
    //TODO: impl
    Command::NotImpletated
}
