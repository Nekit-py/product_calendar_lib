use crate::day::{kind::DayKind, Day};
use chrono::NaiveDate;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;

const URL: &str = "https://www.consultant.ru/law/ref/calendar/proizvodstvennye";
const MONTHS: [&str; 12] = [
    "Январь",
    "Февраль",
    "Март",
    "Апрель",
    "Май",
    "Июнь",
    "Июль",
    "Август",
    "Сентябрь",
    "Октябрь",
    "Ноябрь",
    "Декабрь",
];

#[derive(Debug)]
pub struct ProductCalendarParser {
    months: HashMap<&'static str, u8>,
    pub year: u16,
    url: String,
}

impl ProductCalendarParser {
    pub fn new(year: u16) -> ProductCalendarParser {
        let months: HashMap<&'static str, u8> = MONTHS
            .iter()
            .enumerate()
            .map(|(i, &m)| (m, i as u8 + 1))
            .collect();

        Self {
            months,
            year,
            url: format!("{}/{}", URL, year),
        }
    }

    fn collect_days(
        &self,
        table: ElementRef,
        selector: &Selector,
        month_number: u8,
        day_kind: DayKind,
        replace_chars: &str,
    ) -> Vec<Day> {
        table
            .select(selector)
            .filter_map(|e| {
                e.text().next().and_then(|day_text| {
                    self.to_date(day_text.replace(replace_chars, ""), month_number)
                        .map(|date| {
                            let mut day = Day::new(date);
                            day.set_kind(day_kind);
                            day
                        })
                })
            })
            .collect()
    }

    pub fn parse_calendar(&mut self) -> Result<Vec<Day>, Box<dyn std::error::Error>> {
        let document = {
            let client = Client::new();
            let resp = client
                .get(&self.url)
                .header(USER_AGENT, "Mozilla/5.0")
                .send()?;
            let body = resp.text()?;
            Html::parse_document(&body)
        };

        let month_selector = Selector::parse(".month")?;
        let holiday_selector = Selector::parse(".holiday")?;
        let preholiday_selector = Selector::parse("td.preholiday")?;
        let work_selector = Selector::parse("td.work")?;

        let mut calendar = Vec::with_capacity(30);

        for table in document.select(&Selector::parse("table")?) {
            if let Some(month_element) = table.select(&month_selector).next() {
                let month_name = month_element.text().collect::<String>().trim().to_string();
                if let Some(&month_number) = self.months.get(month_name.as_str()) {
                    calendar.extend(self.collect_days(
                        table,
                        &holiday_selector,
                        month_number,
                        DayKind::Holiday,
                        "\u{a0}",
                    ));
                    calendar.extend(self.collect_days(
                        table,
                        &preholiday_selector,
                        month_number,
                        DayKind::Preholiday,
                        "*\u{a0}",
                    ));
                    calendar.extend(self.collect_days(
                        table,
                        &work_selector,
                        month_number,
                        DayKind::Work,
                        "\u{a0}",
                    ));
                }
            }
        }
        Ok(calendar)
    }

    fn to_date(&self, day: String, month: u8) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year as i32, month as u32, day.parse().ok()?)
    }
}

mod tests {

    #[test]
    fn test_parse_calendar() {
        let mut parser = super::ProductCalendarParser::new(2024);
        let parsed_calendar = parser
            .parse_calendar()
            .expect("Не удалось распарсить календарь");

        let mut expected_day =
            super::Day::new(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        expected_day.set_kind(super::DayKind::Holiday);

        assert_eq!(parsed_calendar[0], expected_day);
    }
}
