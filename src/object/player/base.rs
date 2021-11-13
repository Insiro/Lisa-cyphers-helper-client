#![allow(non_snake_case)]
use super::Player;
use super::PlayerBuilder;
use crate::error as lisa_error;
use crate::object::{neople, Object};
use crate::player_impl_neople;
use crate::util::temp;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PlayerBaseList {
    pub rows: Vec<Base>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct Base {
    playerId: String,
    nickname: String,
    pub grade: u8,
}
impl neople::Object for Base {}
impl Object for Base {}

player_impl_neople!(Base);

impl Base {
    pub fn from_player(player: &Box<dyn Player>) -> Self {
        let p = player.as_ref();
        Self {
            nickname: p.get_name().to_string(),
            playerId: p.get_id().to_string(),
            grade: p.get_grade(),
        }
    }
}

pub struct BaseBuilder {
    id: String,
}

impl PlayerBuilder for BaseBuilder {
    type Player = Base;
    fn new(id: String) -> Result<Self, Box<(dyn std::error::Error)>> {
        Ok(Self { id })
    }

    fn build(&self) -> Result<Self::Player, Box<dyn std::error::Error>> {
        let p = temp::parse::player(&self.id);
        let obj: PlayerBaseList = serde_json::from_str(p.as_str())?;

        let player = Self::get_player(obj)?;
        Ok(player)
    }
}
impl BaseBuilder {
    pub fn from_name(name: &str) -> Result<Base, Box<dyn std::error::Error>> {
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
        Ok(row[0].clone())
    }
}
