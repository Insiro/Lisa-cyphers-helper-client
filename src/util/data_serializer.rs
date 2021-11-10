use crate::util::UtcTime;
use chrono::{TimeZone, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};
const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
pub mod option_date_se {
    use super::*;

    type OptionUtcTime = Option<UtcTime>;

    pub fn serialize<S>(date: &OptionUtcTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match date {
            None => "None".to_string(),
            Some(date) => format!("{}", date.format(FORMAT)),
        };
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OptionUtcTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.eq("None") {
            return Ok(None);
        }
        match Utc
            .datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
        {
            Ok(s) => Ok(Some(s)),
            Err(s) => Err(s),
        }
    }
}

pub mod date_se {
    use super::*;

    pub fn serialize<S>(date: &UtcTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ret = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&ret)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<UtcTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
