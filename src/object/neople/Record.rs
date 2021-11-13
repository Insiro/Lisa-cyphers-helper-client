#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Record {
    pub gameTypeId: String,
    pub winCount: u8,
    pub loseCount: u8,
    pub stopCount: u8,
}

impl Record {
    pub fn dumy() -> Self {
        Self {
            gameTypeId: "dumy".to_string(),
            winCount: 0,
            loseCount: 0,
            stopCount: 0,
        }
    }
}
