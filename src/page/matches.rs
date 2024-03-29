use crate::object::{charactor::Charactor, player};

use super::PageTrait;
pub struct PlayerInfo {
    player: player::Info,
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
            player: player::Info::dumy(),
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
    pub fn search(_id: &str) -> PlayerInfo {
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
    pub fn cli(&self) {}
}
pub fn get_match(_id: &str) -> MatchInfo {
    MatchInfo::dumy()
}

impl PageTrait for MatchInfo {
    fn load(_key: String) -> Self {
        todo!()
    }

    fn get_key(&self) -> String {
        todo!()
    }
}
