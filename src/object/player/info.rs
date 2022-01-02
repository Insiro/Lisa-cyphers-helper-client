#![allow(non_snake_case)]
use super::{Player, PlayerBuilder};
use crate::object::record;
use crate::object::{builder, neople, Object};
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
    id: Option<String>,
}

impl builder::Builder for InfoBuilder {
    type Target = Info;

    fn new() -> Self {
        Self { id: None }
    }
    fn build(&mut self) -> std::result::Result<Self::Target, Box<dyn std::error::Error>> {
        match &self.id {
            Some(id) => {
                let parse_player = temp::parse::player(&id);
                let player: Self::Target = serde_json::from_str(parse_player.as_str())?;
                Ok(player)
            }
            None => Err(Box::new(builder::Error::new(
                "ID not Initialized".to_string(),
            ))),
        }
    }
}

impl PlayerBuilder for InfoBuilder {
    fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }
}

impl InfoBuilder {}
