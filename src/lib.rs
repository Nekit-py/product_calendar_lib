pub mod day;
pub mod parser;
use day::{Day, DayKind};
use parser::ProductCalendarParser;
use thiserror::Error;

use chrono::{Datelike, Duration, NaiveDate, Weekday};

#[derive(Error, Debug)]
pub enum InvalidYearError {
    #[error("Год не может быть установлен ранее 2015")]
    TooYearly,
    #[error("Год не может быть установлен позже текущего.")]
    TooLate,
}

pub struct ProductCalendar {
    calendar: Vec<Day>,
}

impl ProductCalendar {
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

    fn merge(&mut self, consultant_data: &mut Vec<Day>) {
        self.calendar.retain_mut(|d| !consultant_data.contains(d));
        self.calendar.append(consultant_data);
        self.calendar.sort_by_key(|d| d.day);
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

pub async fn get_product_calendar(
    year: Option<u16>,
) -> Result<ProductCalendar, Box<dyn std::error::Error>> {
    let year = validate_year(year)?;

    let mut parser = ProductCalendarParser::new(year);
    let mut conulstant_data = parser.parse_calendar().await?;

    let mut prod_cal = ProductCalendar::new(year);
    prod_cal.merge(&mut conulstant_data);
    println!("{:?}", prod_cal.calendar[0]);
    Ok(prod_cal)
}

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
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

    #[tokio::test]
    async fn test_get_product_calendar() {
        let year = Some(2024);
        match get_product_calendar(year).await {
            Ok(pc) => assert_eq!(
                pc.calendar[0],
                Day {
                    weekday: Weekday::Mon,
                    day: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                    kind: DayKind::Holiday
                }
            ),
            Err(e) => println!("Тест не прошел: {:?}", e),
        }
    }
}
