#![allow(dead_code, non_snake_case)]

use crate::error as lisa_error;
use crate::util::temp::parse;
use serde::Deserialize;
use serde_json;
extern crate getset;
use getset::Getters;
#[derive(Deserialize, Getters)]
pub struct Play {
    #[getset(get)]
    result: String,
    #[getset(get)]
    random: bool,
    #[getset(get)]
    partyUserCount: u8,
    #[getset(get)]
    partyInfo: Party,
    #[getset(get)]
    playTypeName: String,
    #[getset(get)]
    characterId: String,
    #[getset(get)]
    characterName: String,
    #[getset(get)]
    level: u8,
    #[getset(get)]
    killCount: u8,
    #[getset(get)]
    deathCount: u8,
    #[getset(get)]
    assistCount: u8,
    #[getset(get)]
    attackPoint: u8,
    #[getset(get)]
    damagePoint: u8,
    #[getset(get)]
    battilePoint: u8,
    #[getset(get)]
    sightPoint: u8,
    #[getset(get)]
    towerAttackPoint: u8,
    #[getset(get)]
    backAttackCount: u8,
    #[getset(get)]
    comboCount: u8,
    #[getset(get)]
    spellCount: u8,
    #[getset(get)]
    healAmount: u8,
    #[getset(get)]
    sentinelKillCount: u8,
    #[getset(get)]
    demolisherKillCount: u8,
    #[getset(get)]
    trooperKillCount: u8,
    #[getset(get)]
    guardianKillCount: u8,
    #[getset(get)]
    guardTowerKillCount: u8,
    #[getset(get)]
    getCoint: u8,
    #[getset(get)]
    spendCoin: u8,
    #[getset(get)]
    playTime: u8,
}
#[derive(Deserialize, Getters)]
pub struct Party {
    #[getset(get)]
    playerId: String,
    #[getset(get)]
    nickname: String,
    #[getset(get)]
    characterId: String,
    #[getset(get)]
    characterName: String,
}
#[derive(Deserialize, Getters)]
pub struct Map {
    #[getset(get)]
    mapId: String,
    #[getset(get)]
    name: String,
}
#[derive(Deserialize, Getters)]
pub struct Position {
    #[getset(get)]
    name: String,
    #[getset(get)]
    explain: String,
    #[getset(get)]
    attribute: Vec<PositionAttr>,
}
#[derive(Deserialize, Getters)]
struct PositionAttr {
    #[getset(get)]
    level: u8,
    #[getset(get)]
    id: String,
    #[getset(get)]
    name: String,
}
#[derive(Deserialize, Getters)]
pub struct Item {
    #[getset(get)]
    itemId: String,
    #[getset(get)]
    itemName: String,
    #[getset(get)]
    slotCode: u8,
    #[getset(get)]
    slotName: String,
    #[getset(get)]
    rarityCode: u8,
    #[getset(get)]
    rarityName: String,
    #[getset(get)]
    equipSlotCode: u8,
    #[getset(get)]
    equipSlotName: String,
}
#[derive(Deserialize,Getters)]
pub struct Character {
    #[getset(get)]characterId: String,
    #[getset(get)]characterName: String,
}
impl Character {
    pub fn get_list() -> lisa_error::Result<Vec<Character>> {
        let parsed = parse::char_list();
        let value: serde_json::Result<sub::CharacterList> = serde_json::from_str(&parsed);
        match value {
            Err(_) => Err(lisa_error::new(
                "failed to parse",
                lisa_error::Kind::DataError,
            )),
            Ok(ok) => Ok(ok.rows),
        }
    }
}
// impl super::Neople for Character {}

mod sub {
    use super::*;
    #[derive(Deserialize)]
    pub struct CharacterList {
        pub rows: Vec<Character>,
    }
}
