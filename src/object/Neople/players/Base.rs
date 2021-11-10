use serde::{Deserialize, Serialize};

use crate::error as lisa_error;
use crate::player_impl;
use crate::util::temp;

use super::Player;
use super::PlayerBuilder;

#[derive(Deserialize)]
struct PlayerBaseList {
    pub rows: Vec<Base>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct Base {
    playerId: String,
    nickname: String,
    grade: u8,
}

player_impl!(Base);
struct BaseBuilder {
    id: String,
}

impl PlayerBuilder for BaseBuilder {
    fn new(id: String) -> Self {
        Self { id }
    }

    fn build(&self) -> Result<Box<dyn Player>, Box<dyn std::error::Error>> {
        let p = temp::parse::player(&self.id);
        let obj: PlayerBaseList = serde_json::from_str(p.as_str())?;

        let player = Self::get_player(obj)?;
        Ok(Box::new(player))
    }
}
impl BaseBuilder {
    fn from_name(name: &str) -> Result<Base, Box<dyn std::error::Error>> {
        let p = temp::parse::player(&name);
        let obj: PlayerBaseList = serde_json::from_str(p.as_str())?;
        Self::get_player(obj)
    }
    fn get_player(players: PlayerBaseList) -> Result<Base, Box<dyn std::error::Error>> {
        let row = players.rows;
        if row.is_empty() {
            return Err(Box::new(lisa_error::new(
                "Not Found",
                lisa_error::Kind::NotFoundError,
            )));
        }
        Ok(row[0])
    }
}
