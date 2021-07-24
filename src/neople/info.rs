use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Play {
    result: String,
    random: bool,
    partyUserCount: u8,
    partyInfo: Party,
    playTypeName: String,
    characterId: String,
    characterName: String,
    level: u8,
    killCount: u8,
    deathCount: u8,
    assistCount: u8,
    attackPoint: u8,
    damagePoint: u8,
    battilePoint: u8,
    sightPoint: u8,
    towerAttackPoint: u8,
    backAttackCount: u8,
    comboCount: u8,
    spellCount: u8,
    healAmount: u8,
    sentinelKillCount: u8,
    demolisherKillCount: u8,
    trooperKillCount: u8,
    guardianKillCount: u8,
    guardTowerKillCount: u8,
    getCoint: u8,
    spendCoin: u8,
    spendConsumablesCoin: u8,
    playTime: u8,
    responseTime: u8,
    minLifeTime: u8,
    maxLifeTime: u8,
}
#[derive(Serialize, Deserialize)]
pub struct Party {
    playerId: String,
    nickname: String,
    characterId: String,
    characterName: String,
}
#[derive(Serialize, Deserialize)]
pub struct Map {
    mapId: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Position {
    name: String,
    explain: String,
    attribute: Vec<PositionAttr>,
}
#[derive(Serialize, Deserialize)]
struct PositionAttr {
    level: u8,
    id: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Item {
    itemId:String,
    itemName:String,
    slotCode:u8,
    slotName:String,
    rarityCode:u8,
    rarityName:String,
    equipSlotCode:u8,
    equipSlotName:String
}
