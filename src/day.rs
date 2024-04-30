use chrono::{NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DayKind {
    Holiday,
    Preholiday,
    Work,
    Weekend,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Day {
    #[serde(with = "weekday")]
    pub weekday: Weekday,
    #[serde(with = "date_format")]
    pub day: NaiveDate,
    pub kind: DayKind,
}

mod weekday {
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
}

mod date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";
    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let d = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(d)
    }
}

impl PartialEq for Day {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day
    }
}

impl Hash for Day {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.day.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let d1 = Day {
            weekday: Weekday::Mon,
            day: NaiveDate::default(),
            kind: DayKind::Work,
        };

        let d2 = Day {
            weekday: Weekday::Sun,
            day: NaiveDate::default(),
            kind: DayKind::Work,
        };

        assert_eq!(d1, d2);
    }
    #[test]
    fn test_serialize() {
        let d1 = Day {
            weekday: Weekday::Mon,
            day: NaiveDate::from_ymd_opt(2024, 5, 6).unwrap(),
            kind: DayKind::Work,
        };
        let serialized = serde_json::to_string(&d1).unwrap();

        println!("serialized = {}", serialized);
    }
}
