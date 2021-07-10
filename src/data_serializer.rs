pub mod date_se {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};
    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    type UtcTime = Option<DateTime<Utc>>;

    pub fn serialize<S>(date: &UtcTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match date {
            None => "None".to_string(),
            Some(date) => format!("{}", date.format(FORMAT)),
        };
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<UtcTime, D::Error>
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
