pub mod deser;
pub mod impls;
pub mod kind;

use self::deser::{date, weekday};
use self::kind::DayKind;
use chrono::{NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::cmp::Eq;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Day {
    #[serde(with = "weekday")]
    pub weekday: Weekday,
    #[serde(with = "date")]
    pub day: NaiveDate,
    pub kind: DayKind,
}

impl Day {
    pub fn new(day: NaiveDate) -> Self {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_day_eq() {
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

    #[test]
    fn test_day_as_map() {
        let d1 = Day {
            weekday: Weekday::Mon,
            day: NaiveDate::from_ymd_opt(2024, 5, 6).unwrap(),
            kind: DayKind::Work,
        };

        let mut day_map = HashMap::with_capacity(3);
        day_map.insert("weekday".to_owned(), "Mon".to_string());
        day_map.insert("day".to_owned(), "2024-05-06".to_string());
        day_map.insert("kind".to_owned(), "Work".to_string());

        println!("{:?}", d1.as_map());
        assert_eq!(d1.as_map(), day_map);
    }
}
