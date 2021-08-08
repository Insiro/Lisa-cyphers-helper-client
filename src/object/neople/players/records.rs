#![allow(dead_code, non_snake_case)]
//use crate::error as lisa_error;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
type UtcTime = Option<DateTime<Utc>>;
// use super::info;
// use getset::Getters;

#[derive(Deserialize, Serialize)]
pub struct Records {
    pub gameTypeId: String,
    pub winCount: u8,
    pub loseCount: u8,
    pub stopCount: u8,
}

pub fn dumy() -> Records {
    Records {
        gameTypeId: "dumy".to_string(),
        winCount: 0,
        loseCount: 0,
        stopCount: 0,
    }
}
