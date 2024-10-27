pub mod deser;
pub mod impls;
pub mod kind;

use self::deser::{date, weekday};
use self::kind::DayKind;
use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::cmp::Eq;

//TODO: Добавить порядковый номер в году?
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Day {
    #[serde(with = "weekday")]
    weekday: Weekday,
    #[serde(with = "date")]
    day: NaiveDate,
    kind: DayKind,
}

impl Day {
    pub fn new(day: NaiveDate) -> Self {
        let weekday = day.weekday();
        let kind = match weekday {
            Weekday::Sat | Weekday::Sun => DayKind::Weekend,
            _ => DayKind::Work,
        };
        Day { day, weekday, kind }
    }

    pub fn get_date(&self) -> NaiveDate {
        self.day
    }

    pub fn get_year(&self) -> i32 {
        self.day.year()
    }

    pub fn get_weekday(&self) -> Weekday {
        self.weekday
    }

    pub fn get_kind(&self) -> DayKind {
        self.kind
    }

    pub fn set_kind(&mut self, kind: DayKind) {
        self.kind = kind;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_get_date() {
        let date = NaiveDate::from_ymd_opt(2024, 5, 6).unwrap();
        let d = Day::new(date);
        assert_eq!(d.get_date(), date);
    }

    #[test]
    fn test_get_year() {
        let date = NaiveDate::from_ymd_opt(2024, 5, 6).unwrap();
        let d = Day::new(date);
        assert_eq!(d.get_year(), 2024);
    }

    #[test]
    fn test_get_kind() {
        let d = Day::new(NaiveDate::from_ymd_opt(2024, 5, 6).unwrap());
        let kind = d.get_kind();
        assert_eq!(DayKind::Work, kind);
    }

    #[test]
    fn test_get_weekday() {
        let d = Day::new(NaiveDate::from_ymd_opt(2024, 5, 6).unwrap());
        let weekday = d.get_weekday();
        assert_eq!(Weekday::Mon, weekday);
    }

    #[test]
    fn test_day_not_eq() {
        let mut d1 = Day::new(NaiveDate::from_ymd_opt(2024, 5, 8).unwrap());
        d1.set_kind(DayKind::Preholiday);

        let d2 = Day::new(NaiveDate::from_ymd_opt(2024, 5, 8).unwrap());
        assert_ne!(d1, d2);
    }

    #[test]
    fn test_serialize() {
        let d1 = Day::new(NaiveDate::from_ymd_opt(2024, 5, 6).unwrap());
        let serialized = serde_json::to_string(&d1).unwrap();
        let expected = r#"{"weekday":"Mon","day":"2024-05-06","kind":"Work"}"#;
        assert_eq!(&serialized[..], expected);
    }

    #[test]
    fn test_day_as_map() {
        let d = Day::new(NaiveDate::from_ymd_opt(2024, 5, 6).unwrap());
        let mut day_map = HashMap::with_capacity(3);
        day_map.insert("weekday".to_owned(), "Mon".to_string());
        day_map.insert("day".to_owned(), "2024-05-06".to_string());
        day_map.insert("kind".to_owned(), "Work".to_string());
        assert_eq!(d.as_map(), day_map);
    }
}
