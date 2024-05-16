use crate::day::{kind::DayKind, Day};
use crate::parser::ProductCalendarParser;
use crate::statistic::Statistic;
use std::collections::HashMap;
use std::ops::Index;
use thiserror::Error;

use chrono::{Datelike, Duration, NaiveDate, Weekday};

#[derive(Error, Debug)]
pub enum InvalidYearError {
    #[error("Год не может быть установлен ранее 2015")]
    TooYearly,
    #[error("Год не может быть установлен позже текущего.")]
    TooLate,
}

#[derive(Clone, Debug)]
pub struct ProductCalendar {
    pub calendar: Vec<Day>,
}

impl FromIterator<Day> for ProductCalendar {
    fn from_iter<T: IntoIterator<Item = Day>>(iter: T) -> Self {
        let calendar: Vec<_> = iter.into_iter().collect();
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
    //Кол-во дней в периоде
    pub fn total_days(&self) -> usize {
        self.calendar.len()
    }

    fn iter(&self) -> impl Iterator<Item = &Day> + '_ {
        self.calendar.iter()
    }

    pub fn by_week_num(&self, num: usize) -> Option<Self> {
        if num > 52 || num >= self.calendar.len() / 7 {
            return None;
        }
        let chunk_size = 7;
        let start_idx = num * chunk_size;
        let end_idx = start_idx + chunk_size;
        Some(Self {
            calendar: self.calendar[start_idx..end_idx].to_vec(),
        })
    }

    // TODO: Удалить?
    //Конвертирует вектр с Day в вектор с мапой
    pub fn as_vec_hashmap(&self) -> Vec<HashMap<String, String>> {
        self.calendar
            .clone()
            .into_iter()
            .map(|d| d.as_map())
            .collect()
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
    //TODO мб вернуть Option, т.к. входная дата может быть неверна и кол-во дней слишком болшое.
    pub fn period_by_number_of_days(self, date: NaiveDate, days: usize) -> Self {
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

    pub fn period_by_number_of_work_days(self, date: NaiveDate, mut work_days: usize) -> Self {
        let start_idx = self
            .iter()
            .position(|d| d.day == date)
            .expect("Данной даты нет в этому году");
        let mut days = 0;
        for d in self.calendar[start_idx..].iter() {
            if work_days == 0 {
                break;
            }
            if d.kind == DayKind::Work || d.kind == DayKind::Preholiday {
                work_days -= 1;
            }
            days += 1;
        }
        let new_calendar = self.calendar[start_idx..start_idx + days].to_vec();

        Self {
            calendar: new_calendar,
        }
    }

    //Опционально возвращает следующий рабочий день.
    pub fn next_work_day(&self, cur_day: NaiveDate) -> Option<Day> {
        let start_idx = self.iter().position(|d| d.day == cur_day);

        if let Some(start_idx) = start_idx {
            for d in self.calendar.iter().skip(start_idx + 1) {
                match d.kind {
                    DayKind::Work | DayKind::Preholiday => return Some(d.clone()),
                    _ => continue,
                }
            }
        }
        None // Возвращаем None, если следующий рабочий день не найден
    }

    //TODO мб вернуть Option, т.к. входная дата может быть неверна
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

    pub fn extract_dates_in_quarter(&self, quarter: u8) -> Option<Self> {
        let first_querater_len = if self.calendar.len() == 366 { 90 } else { 89 };
        match quarter {
            1 => Some(Self {
                calendar: self.calendar[..first_querater_len + 1].to_vec(),
            }),
            2 => Some(Self {
                calendar: self.calendar[first_querater_len + 1..first_querater_len + 92].to_vec(),
            }),
            3 => Some(Self {
                calendar: self.calendar[first_querater_len + 92..first_querater_len + 184].to_vec(),
            }),

            4 => Some(Self {
                calendar: self.calendar[first_querater_len + 184..self.calendar.len()].to_vec(),
            }),
            _ => None,
        }
    }

    pub fn holidays(&self) -> Self {
        self.clone()
            .into_iter()
            .filter(|day| day.kind == DayKind::Holiday)
            .collect()
    }

    //Подсчет статистики
    pub fn statistic(&self) -> Statistic {
        let mut statistic = Statistic::default();

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

fn validate_year(year: Option<u16>) -> Result<u16, InvalidYearError> {
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

pub fn get_product_calendar(
    year: Option<u16>,
) -> Result<ProductCalendar, Box<dyn std::error::Error>> {
    let year = validate_year(year)?;

    let mut parser = ProductCalendarParser::new(year);
    let mut consultant_data = parser.parse_calendar()?;

    let mut prod_cal = ProductCalendar::new(year);
    prod_cal.merge(&mut consultant_data);
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
                        .period_by_number_of_days(NaiveDate::from_ymd_opt(2024, 5, 6).unwrap(), 3)
                        .calendar
                );
                assert_eq!(
                    pc.clone()
                        .period_by_number_of_days(NaiveDate::from_ymd_opt(2024, 5, 6).unwrap(), 3)
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
                );
                // println!(
                //     "{:?}",
                //     pc.clone()
                //         .period_by_number_of_days(NaiveDate::from_ymd_opt(2024, 5, 1).unwrap(), 30)
                // );
                // println!("{:#?}", pc.clone().extract_dates_in_quarter(1));
                assert_eq!(
                    pc.clone()
                        .period_by_number_of_work_days(
                            NaiveDate::from_ymd_opt(2024, 6, 11).unwrap(),
                            2
                        )
                        .calendar,
                    vec![
                        Day {
                            weekday: Weekday::Tue,
                            day: NaiveDate::from_ymd_opt(2024, 6, 11).unwrap(),
                            kind: DayKind::Preholiday,
                        },
                        Day {
                            weekday: Weekday::Wed,
                            day: NaiveDate::from_ymd_opt(2024, 6, 12).unwrap(),
                            kind: DayKind::Holiday,
                        },
                        Day {
                            weekday: Weekday::Thu,
                            day: NaiveDate::from_ymd_opt(2024, 6, 13).unwrap(),
                            kind: DayKind::Work,
                        },
                    ]
                );
            }
            Err(e) => println!("Тест не прошел: {:?}", e),
        }
    }
}
