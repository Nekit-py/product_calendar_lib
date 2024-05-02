use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DayKind {
    Holiday,
    Preholiday,
    Work,
    Weekend,
}

impl fmt::Display for DayKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DayKind::Holiday => write!(f, "Holiday"),
            DayKind::Preholiday => write!(f, "Preholiday"),
            DayKind::Work => write!(f, "Work"),
            DayKind::Weekend => write!(f, "Weekend"),
        }
    }
}
