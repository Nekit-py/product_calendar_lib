use crate::day::{Day, DayKind};
use chrono::{Datelike, NaiveDate};
use reqwest::Error;
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
            .map(|e| {
                let day_text = e.text().next().unwrap();
                let date = self.to_date(day_text.replace(replace_chars, ""), month_number);
                Day {
                    weekday: date.weekday(),
                    day: date,
                    kind: day_kind,
                }
            })
            .collect()
    }

    pub async fn parse_calendar(&mut self) -> Result<Vec<Day>, Error> {
        let resp = reqwest::get(&self.url).await?;
        let body = resp.text().await?;
        let document = Html::parse_document(&body);

        let mut calendar: Vec<Day> = Vec::with_capacity(30);

        let month_selector = Selector::parse(".month").unwrap();
        let holiday_selector = Selector::parse(".holiday").unwrap();
        let preholiday_selector = Selector::parse("td.preholiday").unwrap();
        let work = Selector::parse("td.work").unwrap();

        for table in document.select(&Selector::parse("table").unwrap()) {
            if let Some(month_element) = table.select(&month_selector).next() {
                let month_name = month_element.text().collect::<String>();
                let month_number = *self.months.get(month_name.as_str()).unwrap_or(&0_u8);

                let holidays = self.collect_days(
                    table,
                    &holiday_selector,
                    month_number,
                    DayKind::Holiday,
                    "\u{a0}",
                );
                let preholidays = self.collect_days(
                    table,
                    &preholiday_selector,
                    month_number,
                    DayKind::Preholiday,
                    "*\u{a0}",
                );
                let work_in_weekend =
                    self.collect_days(table, &work, month_number, DayKind::Work, "\u{a0}");

                calendar.extend(holidays);
                calendar.extend(preholidays);
                calendar.extend(work_in_weekend);
            }
        }
        Ok(calendar)
    }

    fn to_date(&self, day: String, month: u8) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year as i32, month as u32, day.parse::<u32>().unwrap())
            .expect("Не удалось разобрать дату...")
    }
}
