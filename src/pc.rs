use crate::day::{Day, DayKind};
use crate::parser::ProductCalendarParser;
use std::collections::HashMap;
use std::ops::Index;
use thiserror::Error;

use chrono::{Datelike, Duration, NaiveDate, ParseError, Weekday};

#[derive(Debug, Clone, Copy)]
pub struct Statistic {
    holidays: u8,
    work_days: u8,
    weekends: u8,
    preholidays: u8,
}

impl Statistic {
    pub fn rest_days(&self) -> u8 {
        self.holidays + self.weekends
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

#[derive(Error, Debug)]
pub enum InvalidYearError {
    #[error("Год не может быть установлен ранее 2015")]
    TooYearly,
    #[error("Год не может быть установлен позже текущего.")]
    TooLate,
}

#[derive(Clone)]
pub struct ProductCalendar {
    calendar: Vec<Day>,
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
    pub fn iter(&self) -> impl Iterator<Item = &Day> + '_ {
        self.calendar.iter()
    }

    //Конвертирует вектр с Day в вектор с мапой
    pub fn as_vec_hasmap(&self) -> Vec<HashMap<String, String>> {
        let mut calendar = Vec::with_capacity(366);
        for day in self.calendar.clone().into_iter() {
            calendar
                .push(serde_json::from_str(serde_json::to_string(&day).unwrap().as_str()).unwrap())
        }
        calendar
    }

    pub fn new(year: u16) -> ProductCalendar {
        let start_date = NaiveDate::from_ymd_opt(year as i32, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(year as i32, 12, 31).unwrap();

        //Максимальная вместимость вектора =
        //кол-ву дней в високосном году
        let mut dates = Vec::with_capacity(366);
        let mut current_date = start_date;

        while current_date <= end_date {
            dates.push(Day {
                day: current_date,
                weekday: current_date.weekday(),
                kind: match current_date.weekday() {
                    Weekday::Sat | Weekday::Sun => DayKind::Weekend,
                    _ => DayKind::Work,
                },
            });
            current_date += Duration::days(1);
        }
        ProductCalendar { calendar: dates }
    }

    //Слияние данных календаря из консультанта и стандартного расписания
    fn merge(&mut self, consultant_data: &mut Vec<Day>) {
        self.calendar.retain_mut(|d| !consultant_data.contains(d));
        self.calendar.append(consultant_data);
        self.calendar.sort_by_key(|d| d.day);
    }

    //Возвращает экземпляр с периодом от указанной даты
    //до указанная дата + кол-во дней ключительно
    pub fn period_by_by_number_of_days(self, date: NaiveDate, days: usize) -> Self {
        let start_idx = self
            .iter()
            .position(|d| d.day == date)
            .expect("Данной даты нет в этому году");
        let end_idx = start_idx + days;
        if end_idx > self.calendar.len() {
            panic!("Запрашиваемый период выходит за пределы календаря");
        }

        let new_calendar = self.calendar[start_idx..end_idx].to_vec();

        Self {
            calendar: new_calendar,
        }
    }

    pub fn period_slice(self, start: NaiveDate, end: NaiveDate) -> Self {
        let start_idx = self
            .iter()
            .position(|d| d.day == start)
            .expect("Данной даты нет в этому году");
        let end_idx = (start - end).num_days() as usize;
        if end_idx > self.calendar.len() {
            panic!("Запрашиваемый период выходит за пределы календаря");
        }

        let new_calendar = self.calendar[start_idx..end_idx].to_vec();

        Self {
            calendar: new_calendar,
        }
    }

    //Подсчет статистики
    pub fn statistic(&self) -> Statistic {
        let mut statistic = Statistic {
            holidays: 0,
            work_days: 0,
            weekends: 0,
            preholidays: 0,
        };

        for day in self {
            match day.kind {
                DayKind::Holiday => statistic.holidays += 1,
                DayKind::Preholiday => statistic.preholidays += 1,
                DayKind::Work => statistic.work_days += 1,
                DayKind::Weekend => statistic.weekends += 1,
            }
        }

        statistic
    }
}

pub fn validate_year(year: Option<u16>) -> Result<u16, InvalidYearError> {
    let cur_year = chrono::Local::now().year() as u16;
    if let Some(y) = year {
        //Производсвтенный календарь в консультанте ведется с 2015 года
        if y < 2015_u16 {
            return Err(InvalidYearError::TooYearly);
        } else if y > cur_year {
            return Err(InvalidYearError::TooLate);
        }
        return Ok(y);
    }
    Ok(cur_year)
}

fn validate_date(date: String) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(&date, "%d.%m.%y")
}

pub fn get_product_calendar(
    year: Option<u16>,
) -> Result<ProductCalendar, Box<dyn std::error::Error>> {
    let year = validate_year(year)?;

    let mut parser = ProductCalendarParser::new(year);
    let mut conulstant_data = parser.parse_calendar()?;

    let mut prod_cal = ProductCalendar::new(year);
    prod_cal.merge(&mut conulstant_data);
    Ok(prod_cal)
}

mod tests {
    use super::*;

    #[test]
    fn test_get_product_calendar() {
        let year = Some(2024);
        match get_product_calendar(year) {
            Ok(pc) => {
                println!(
                    "{:?}",
                    pc.clone()
                        .period_by_by_number_of_days(
                            NaiveDate::from_ymd_opt(2024, 5, 6).unwrap(),
                            3
                        )
                        .calendar
                );
                assert_eq!(
                    pc.clone()
                        .period_by_by_number_of_days(
                            NaiveDate::from_ymd_opt(2024, 5, 6).unwrap(),
                            3
                        )
                        .calendar,
                    [
                        Day {
                            weekday: Weekday::Mon,
                            day: NaiveDate::from_ymd_opt(2024, 5, 6).unwrap(),
                            kind: DayKind::Work
                        },
                        Day {
                            weekday: Weekday::Tue,
                            day: NaiveDate::from_ymd_opt(2024, 5, 7).unwrap(),
                            kind: DayKind::Work
                        },
                        Day {
                            weekday: Weekday::Wed,
                            day: NaiveDate::from_ymd_opt(2024, 5, 8).unwrap(),
                            kind: DayKind::Preholiday
                        }
                    ]
                );
                assert_eq!(
                    pc.clone().statistic(),
                    Statistic {
                        holidays: 17,
                        work_days: 243,
                        weekends: 101,
                        preholidays: 5
                    }
                );
                assert_eq!(
                    pc.clone()[0],
                    Day {
                        weekday: Weekday::Mon,
                        day: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                        kind: DayKind::Holiday
                    }
                )
            }
            Err(e) => println!("Тест не прошел: {:?}", e),
        }
    }
}
