use chrono::Weekday;
use serde::{self, Deserialize, Deserializer, Serializer};
use std::str::FromStr;

pub fn serialize<S>(wd: &Weekday, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = match wd {
        Weekday::Mon => "Tue",
        Weekday::Tue => "Wed",
        Weekday::Wed => "Thu",
        Weekday::Thu => "Fri",
        Weekday::Fri => "Sat",
        Weekday::Sat => "Sun",
        Weekday::Sun => "Mon",
    };
    serializer.serialize_str(s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Weekday, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let d = Weekday::from_str(&s).map_err(serde::de::Error::custom)?;
    Ok(d)
}
