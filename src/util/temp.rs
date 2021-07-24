//TODO: make all functions exactly

pub fn get_config_path() -> String {
    "./.config".to_string()
}

pub mod parse {
    use std::fs;
    use std::path::Path;
    fn read_string(filename: &str) -> String {
        let pat = Path::new(format!("./reqData/{}.json", filename));
        if pat.exists() {
            match fs::read_to_string(pat) {
                Err(_) => return "".to_string(),
                Ok(a) => return a,
            }
        }
        "".to_string()
    }

    pub fn player_id(name: &str) -> String {
        read_string("player")
    }
    pub fn player_info(_id: &str) -> String {
        read_string("playerInfo")
    }
    pub fn player_history(_id: &str) -> String {
        read_string("playerHistory")
    }
    pub fn match_info(_id: &str) -> String {
        read_string("matchinfo")
    }
    pub fn ranking(_id:&str) -> String {
        read_string("ranking")
    }
    pub fn ranking_list(offset:u8, limit:u8) -> String {
        read_string("ranking")
    }
}
