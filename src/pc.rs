use crate::day::{kind::DayKind, Day};
use crate::errors::ProductCalendarError;
use crate::parser::ProductCalendarParser;
use crate::statistic::Statistic;
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use std::collections::HashMap;
use std::ops::Index;
use std::sync::Mutex;

lazy_static! {
    static ref CACHED_CALENDAR: Mutex<HashMap<u16, ProductCalendar>> = Mutex::new(HashMap::new());
}

#[derive(Clone, Debug)]
pub struct ProductCalendar {
    pub calendar: Vec<Day>,
}

impl FromIterator<Day> for ProductCalendar {
    fn from_iter<T: IntoIterator<Item = Day>>(iter: T) -> Self {
        let calendar = iter.into_iter().collect();
        ProductCalendar { calendar }
    }
}

impl Index<usize> for ProductCalendar {
    type Output = Day;

    fn index(&self, index: usize) -> &Self::Output {
        &self.calendar[index]
    }
}

impl IntoIterator for ProductCalendar {
    type Item = Day;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.calendar.into_iter()
    }
}

impl<'a> IntoIterator for &'a ProductCalendar {
    type Item = &'a Day;
    type IntoIter = std::slice::Iter<'a, Day>;

    fn into_iter(self) -> Self::IntoIter {
        self.calendar.iter()
    }
}

impl ProductCalendar {
    pub fn total_days(&self) -> usize {
        self.calendar.len()
    }

    fn iter(&self) -> impl Iterator<Item = &Day> {
        self.calendar.iter()
    }

    pub fn after_nth_weeks(
        &self,
        date: NaiveDate,
        weeks: usize,
    ) -> Result<Day, ProductCalendarError> {
        let start_idx = self.iter().position(|d| d.get_date() == date);
        match start_idx {
            Some(start_idx) => {
                let end_idx = start_idx + weeks * 7;
                if end_idx >= self.calendar.len() {
                    return Err(ProductCalendarError::ExceedMaxDaysError(end_idx));
                }
                Ok(self.calendar[end_idx].clone())
            }
            None => Err(ProductCalendarError::DateOutOfRange(date.to_string())),
        }
    }

    pub fn new(year: u16) -> ProductCalendar {
        let start_date = NaiveDate::from_ymd_opt(year as i32, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(year as i32, 12, 31).unwrap();

        let calendar = (0..)
            .map(|i| start_date + Duration::days(i))
            .take_while(|&date| date <= end_date)
            .map(|date| {
                let kind = match date.weekday() {
                    Weekday::Sat | Weekday::Sun => DayKind::Weekend,
                    _ => DayKind::Work,
                };
                let mut d = Day::new(date);
                d.set_kind(kind);
                d
            })
            .collect();

        ProductCalendar { calendar }
    }

    fn merge(&mut self, consultant_data: &mut Vec<Day>) {
        self.calendar.retain(|d| {
            !consultant_data
                .iter()
                .any(|cd| d.get_date() == cd.get_date())
        });
        self.calendar.append(consultant_data);
        self.calendar.sort_by_key(|d| d.get_date());
    }

    pub fn period_by_number_of_days(
        &self,
        date: NaiveDate,
        days: usize,
    ) -> Result<Self, ProductCalendarError> {
        let start_idx = self.iter().position(|d| d.get_date() == date);

        match start_idx {
            Some(start_idx) => {
                let end_idx = start_idx + days;
                if end_idx > self.calendar.len() {
                    return Err(ProductCalendarError::ExceedMaxDaysError(end_idx));
                }
                Ok(Self {
                    calendar: self.calendar[start_idx..end_idx].to_vec(),
                })
            }
            None => Err(ProductCalendarError::DateOutOfRange(date.to_string())),
        }
    }

    pub fn period_by_number_of_work_days(
        &self,
        date: NaiveDate,
        mut work_days: usize,
    ) -> Result<Self, ProductCalendarError> {
        let start_idx = self.iter().position(|d| d.get_date() == date);

        match start_idx {
            Some(start_idx) => {
                let mut days = 0;
                for d in self.calendar[start_idx..].iter() {
                    if work_days == 0 {
                        break;
                    }
                    if d.get_kind() == DayKind::Work || d.get_kind() == DayKind::Preholiday {
                        work_days -= 1;
                    }
                    days += 1;
                }
                let end_idx = start_idx + days;
                if end_idx > self.calendar.len() {
                    return Err(ProductCalendarError::ExceedMaxDaysError(end_idx));
                }
                Ok(Self {
                    calendar: self.calendar[start_idx..end_idx].to_vec(),
                })
            }
            None => Err(ProductCalendarError::DateOutOfRange(date.to_string())),
        }
    }

    pub fn next_work_day(&self, cur_day: NaiveDate) -> Result<Day, ProductCalendarError> {
        let start_idx = self.iter().position(|d| d.get_date() == cur_day);

        if let Some(start_idx) = start_idx {
            for d in self.calendar.iter().skip(start_idx + 1) {
                if matches!(d.get_kind(), DayKind::Work | DayKind::Preholiday) {
                    return Ok(d.clone());
                }
            }
        }
        Err(ProductCalendarError::DateOutOfRange(cur_day.to_string()))
    }

    pub fn period_slice(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Self, ProductCalendarError> {
        let start_idx = self.iter().position(|d| d.get_date() == start);
        let end_idx = self.iter().position(|d| d.get_date() == end);

        match (start_idx, end_idx) {
            (Some(start_idx), Some(end_idx)) => {
                if start_idx > end_idx {
                    return Err(ProductCalendarError::DateOutOfRange(start.to_string()));
                }
                Ok(Self {
                    calendar: self.calendar[start_idx..=end_idx].to_vec(),
                })
            }
            _ => Err(ProductCalendarError::DateOutOfRange(format!(
                "{} - {}",
                start, end
            ))),
        }
    }

    pub fn extract_dates_in_quarter(&self, quarter: u8) -> Result<Self, ProductCalendarError> {
        let first_quarter_len = if self.calendar.len() == 366 { 90 } else { 89 };
        match quarter {
            1 => Ok(Self {
                calendar: self.calendar[..=first_quarter_len].to_vec(),
            }),
            2 => Ok(Self {
                calendar: self.calendar[first_quarter_len + 1..first_quarter_len + 92].to_vec(),
            }),
            3 => Ok(Self {
                calendar: self.calendar[first_quarter_len + 92..first_quarter_len + 184].to_vec(),
            }),

            4 => Ok(Self {
                calendar: self.calendar[first_quarter_len + 184..].to_vec(),
            }),
            _ => Err(ProductCalendarError::InvalidQuarter(quarter)),
        }
    }

    pub fn by_kind(&self, kind: DayKind) -> Self {
        Self {
            calendar: self
                .calendar
                .iter()
                .filter(|day| day.get_kind() == kind)
                .cloned()
                .collect(),
        }
    }

    pub fn statistic(&self) -> Statistic {
        let mut statistic = Statistic::default();

        for day in self.iter() {
            match day.get_kind() {
                DayKind::Holiday => statistic.holidays += 1,
                DayKind::Preholiday => statistic.preholidays += 1,
                DayKind::Work => statistic.work_days += 1,
                DayKind::Weekend => statistic.weekends += 1,
            }
        }
        statistic
    }

    pub fn info_by_date(&self, date: NaiveDate) -> Option<Day> {
        self.iter().find(|day| date == day.get_date()).cloned()
    }
}

fn validate_year(year: Option<u16>) -> Result<u16, ProductCalendarError> {
    const MIN_YEAR: u16 = 2015;
    let current_year = Local::now().year() as u16;
    match year {
        Some(year_value) if year_value >= MIN_YEAR && year_value <= current_year => Ok(year_value),
        Some(year_value) => Err(ProductCalendarError::InvalidYear(year_value.to_string())),
        None => Ok(current_year),
    }
}

pub fn get_product_calendar(
    year: Option<u16>,
) -> Result<ProductCalendar, Box<dyn std::error::Error>> {
    let year = validate_year(year)?;

    let mut parser = ProductCalendarParser::new(year);

    let mut cached = CACHED_CALENDAR.lock().unwrap();

    if let Some(cached_cal) = cached.get(&year) {
        return Ok(cached_cal.clone());
    }

    let mut consultant_data = parser.parse_calendar()?;
    let mut prod_cal = ProductCalendar::new(year);
    prod_cal.merge(&mut consultant_data);
    cached.insert(year, prod_cal.clone());

    Ok(prod_cal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_period_by_number_of_days() {
        let year = Some(2024);
        let pc = get_product_calendar(year).unwrap();

        let expected = {
            let d1 = Day::new(NaiveDate::from_ymd_opt(2024, 5, 6).unwrap());
            let d2 = Day::new(NaiveDate::from_ymd_opt(2024, 5, 7).unwrap());
            let mut d3 = Day::new(NaiveDate::from_ymd_opt(2024, 5, 8).unwrap());
            d3.set_kind(DayKind::Preholiday);
            vec![d1, d2, d3]
        };

        assert_eq!(
            pc.period_by_number_of_days(NaiveDate::from_ymd_opt(2024, 5, 6).unwrap(), 3)
                .unwrap()
                .calendar,
            expected
        );
    }

    #[test]
    fn test_statistic() {
        let year = Some(2024);
        let pc = get_product_calendar(year).unwrap();
        let expected = Statistic {
            holidays: 17,
            work_days: 243,
            weekends: 101,
            preholidays: 5,
        };
        assert_eq!(pc.statistic(), expected);
    }

    #[test]
    fn test_info_by_day() {
        let year = Some(2024);
        let pc = get_product_calendar(year).unwrap();
        let mut d = Day::new(NaiveDate::from_ymd_opt(2024, 6, 11).unwrap());
        d.set_kind(DayKind::Preholiday);
        let test_day = pc
            .info_by_date(NaiveDate::from_ymd_opt(2024, 6, 11).unwrap())
            .unwrap();
        assert_eq!(d, test_day);
    }

    #[test]
    fn test_period_by_number_of_work_days() {
        let year = Some(2024);
        let pc = get_product_calendar(year).unwrap();

        let mut d1 = Day::new(NaiveDate::from_ymd_opt(2024, 6, 11).unwrap());
        d1.set_kind(DayKind::Preholiday);
        let mut d2 = Day::new(NaiveDate::from_ymd_opt(2024, 6, 12).unwrap());
        d2.set_kind(DayKind::Holiday);
        let d3 = Day::new(NaiveDate::from_ymd_opt(2024, 6, 13).unwrap());

        let expected = vec![d1, d2, d3];
        assert_eq!(
            expected,
            pc.period_by_number_of_work_days(NaiveDate::from_ymd_opt(2024, 6, 11).unwrap(), 2)
                .unwrap()
                .calendar,
        );
    }
}
