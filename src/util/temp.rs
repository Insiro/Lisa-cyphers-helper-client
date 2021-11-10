#![allow(dead_code, non_snake_case)]

use super::UtcTime;

//TODO: make all functions exactly

pub fn get_config_path() -> String {
    "./.config/".to_string()
}

pub mod parse {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn read_string(filename: &str) -> String {
        let dir = format!("./reqData/{}.json", filename);
        let pat = Path::new(&dir);
        if pat.exists() {
            match fs::read_to_string(pat) {
                Err(_) => return "".to_string(),
                Ok(a) => return a,
            }
        }
        "".to_string()
    }

    pub fn player_id(_name: &str) -> String {
        read_string("player")
    }
    pub fn player_info(_id: &str) -> String {
        read_string("playerInfo")
    }
    pub fn player_history(
        _id: &str,
        _isNormal: bool,
        _startDate: &Option<UtcTime>,
        _endDate: &Option<UtcTime>,
        _limit: u8,
    ) -> String {
        read_string("playerHistory")
    }
    pub fn match_info(_id: &str) -> String {
        read_string("matchinfo")
    }
    pub fn ranking(_id: &str) -> String {
        read_string("ranking")
    }
    pub fn ranking_list(_offset: u8, _limit: u8) -> String {
        read_string("ranking")
    }
    pub fn char_list() -> String {
        read_string("charlist")
    }
}
