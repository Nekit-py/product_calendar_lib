use super::Day;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt;

impl Day {
    pub fn as_map(&self) -> HashMap<String, String> {
        let mut day_map = HashMap::with_capacity(3);
        day_map.insert("weekday".to_owned(), self.weekday.to_string());
        day_map.insert("day".to_owned(), format!("{}", self.day.format("%Y-%m-%d")));
        day_map.insert("kind".to_owned(), self.kind.to_string());
        day_map
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Day(day={}, kind={}, weekday={})",
            self.day, self.kind, self.weekday
        )
    }
}

impl PartialEq for Day {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day && self.kind == other.kind && self.weekday == other.weekday
    }
}
