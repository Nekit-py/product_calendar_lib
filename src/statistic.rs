use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, Default, Eq)]
pub struct Statistic {
    pub holidays: u16,
    pub work_days: u16,
    pub weekends: u16,
    pub preholidays: u16,
}

impl Statistic {
    pub fn rest_days(&self) -> u16 {
        self.holidays + self.weekends
    }

    //Рабочий день 8 часов
    //Предпраздничный день 7 часов
    pub fn work_hours(&self) -> u16 {
        self.work_days * 8 + self.preholidays * 7
    }

    pub fn as_map(&self) -> HashMap<String, u16> {
        let mut day_map = HashMap::with_capacity(3);
        day_map.insert("holidays".to_owned(), self.holidays);
        day_map.insert("workdays".to_owned(), self.work_days);
        day_map.insert("weekends".to_owned(), self.weekends);
        day_map.insert("prelolidays".to_owned(), self.preholidays);
        day_map
    }
}

impl PartialEq for Statistic {
    fn eq(&self, other: &Self) -> bool {
        self.holidays == other.holidays
            && self.work_days == other.work_days
            && self.weekends == other.weekends
            && self.preholidays == other.preholidays
    }
}

impl fmt::Display for Statistic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Statistic(holidays={}, work_days={}, weekends={}, preholidays={})",
            self.holidays, self.work_days, self.weekends, self.preholidays
        )
    }
}
