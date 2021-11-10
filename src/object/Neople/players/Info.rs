use super::{Player, PlayerBuilder};
use crate::error as lisa_error;
use crate::object;
use crate::object::Neople::Record;
use crate::player_impl;
use crate::util::temp;
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Deserialize, Serialize)]
pub struct Info {
    playerId: String,
    nickname: String,
    grade: u8,
    clanName: String,
    ratingPoint: Option<u8>,
    maxRatingPoint: Option<u8>,
    tierName: Option<String>,
    records: Vec<Record::Record>,
}
player_impl!(Info);

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
            records: vec![Record::Record::dumy()],
        }
    }
}

struct Builder {
    id: String,
}
impl PlayerBuilder for Builder {
    fn new(id: String) -> Self {
        todo!()
    }

    fn build(&self) -> Result<Box<dyn Player>, Box<dyn std::error::Error>> {
        let parsed = temp::parse::player_info(&self.id);
        let se: serde_json::Result<Info> = serde_json::from_str(&parsed);
        match se {
            Err(_) => Err(Box::new(lisa_error::new(
                "parse failed",
                lisa_error::Kind::DataError,
            ))),
            Ok(ok) => Ok(Box::new(ok)),
        }
    }
}
