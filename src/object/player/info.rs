#![allow(non_snake_case)]
use super::{Player, PlayerBuilder};
use crate::error as lisa_error;
use crate::object::record;
use crate::object::{neople, Object};
use crate::player_impl_neople;
use crate::util::temp;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct Info {
    playerId: String,
    nickname: String,
    pub grade: u8,
    clanName: String,
    ratingPoint: Option<u8>,
    maxRatingPoint: Option<u8>,
    tierName: Option<String>,
    records: Vec<record::Record>,
}
impl neople::Object for Info {}
impl Object for Info {}
player_impl_neople!(Info);
impl Info {
    pub fn dumy() -> Info {
        Info {
            playerId: "dumy".to_string(),
            nickname: "dumy".to_string(),
            grade: 0,
            clanName: "dumy".to_string(),
            ratingPoint: None,
            maxRatingPoint: None,
            tierName: None,
            records: vec![record::Record::dumy()],
        }
    }
}

pub struct InfoBuilder {
    id: String,
}
impl PlayerBuilder for InfoBuilder {
    type Player = Info;
    fn new(id: String) -> Result<Self, Box<(dyn std::error::Error)>> {
        Ok(Self { id })
    }

    fn build(&self) -> Result<Info, Box<dyn std::error::Error>> {
        let parsed = temp::parse::player_info(&self.id);
        let se: serde_json::Result<Info> = serde_json::from_str(&parsed);
        match se {
            Err(_) => Err(Box::new(lisa_error::new(
                "parse failed",
                lisa_error::Kind::DataError,
            ))),
            Ok(ok) => Ok(ok),
        }
    }
}
