#![allow(dead_code, non_snake_case)]
use super::info;
pub struct Matches {
    date: String,
    gameTypeId: String,
    map: info::Map,
    teams: Vec<Team>,
    players: Vec<Player>,
}
struct Team {
    result: String,
    players: Vec<String>,
}
pub struct Player {
    playerId: String,
    nickname: String,
    map: info::Map,
    playInfo: info::Play,
    position: info::Position,
    items: Vec<info::Item>,
}
impl Matches {
    fn get_members(_is_win: bool) -> Vec<String> {
        //TODO:fill
        return vec![];
    }
}
