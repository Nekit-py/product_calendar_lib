use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

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

impl FromStr for DayKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Holiday" => Ok(DayKind::Holiday),
            "Preholiday" => Ok(DayKind::Preholiday),
            "Work" => Ok(DayKind::Work),
            "Weekend" => Ok(DayKind::Weekend),
            _ => Err(format!("Invalid DayKind: '{}'. Available options: 'Holiday', 'Preholiday', 'Work', 'Weekend'", s)),
        }
    }
}
