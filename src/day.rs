use chrono::{NaiveDate, Weekday};
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayKind {
    Holiday,
    Preholiday,
    Work,
    Weekend,
}

#[derive(Debug, Clone, Eq)]
pub struct Day {
    pub weekday: Weekday,
    pub day: NaiveDate,
    pub kind: DayKind,
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
