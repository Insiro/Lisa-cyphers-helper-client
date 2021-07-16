use crate::object::{charactor::Charactor, player::ParsedPlayer};
pub struct PlayerInfo {
    player: ParsedPlayer,
    kill: u8,
    death: u8,
    assist: u8,
    attack_point: u8,
    heal_point: u8,
    damage_point: u8,
    level: u8,
    charactor: Charactor,
}

impl PlayerInfo {
    pub fn dumy() -> PlayerInfo {
        PlayerInfo {
            player: ParsedPlayer::dumy(),
            attack_point: 0,
            kill: 0,
            death: 0,
            assist: 0,
            heal_point: 0,
            damage_point: 0,
            level: 0,
            charactor: Charactor::dumy(),
        }
    }
    pub fn search(id: &str) -> PlayerInfo {
        PlayerInfo::dumy()
    }
}

struct Team {
    name: String,
    avg_rp: u8,
    players: [PlayerInfo; 5],
}
impl Team {
    pub fn dumy() -> Team {
        Team {
            name: "".to_string(),
            avg_rp: 0,
            players: [
                PlayerInfo::dumy(),
                PlayerInfo::dumy(),
                PlayerInfo::dumy(),
                PlayerInfo::dumy(),
                PlayerInfo::dumy(),
            ],
        }
    }
}
pub struct MatchInfo {
    team_a: Team,
    team_b: Team,
    avg_rp: u8,
    match_id: String,
}
impl MatchInfo {
    pub fn dumy() -> MatchInfo {
        MatchInfo {
            team_a: Team::dumy(),
            team_b: Team::dumy(),
            avg_rp: 0,
            match_id: "".to_string(),
        }
    }
}
pub fn get_match(id: &str) -> MatchInfo {
    MatchInfo::dumy()
}


pub fn cli( args:&mut Vec<&str>, position:u8) {}

pub fn cls() {
    loop {
        let query = "".to_string();
        match query.as_str() {
            "get" => {
                get_match("");
            }
            "back" => break,
            _ => break,
        }
    }
}
