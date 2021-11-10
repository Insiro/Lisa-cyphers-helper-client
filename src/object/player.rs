use super::Neople;
use crate::error as lisa_error;
use serde::{Deserialize, Serialize};
pub use Neople::players::Info;

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

pub trait PlayerChangerble {
    fn set_nickname(&mut self, player_name: &str) -> bool;
    fn set_player_id(&mut self, id: &str) -> lisa_error::Result<()>;
}
#[derive(Serialize, Deserialize)]
pub struct PlayerBase {
    player_name: String,
    player_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ParseBase {
    player_id: String,
    nickname: String,
    grade: u8,
}
