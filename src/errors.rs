use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProductCalendarError {
    #[error("Данные на `{0}` год недоступны.")]
    InvalidYear(String),
    #[error("Дата `{0}` нахидтся вне диапазона текущего производственного календаря.")]
    DateOutOfRange(String),
    #[error("Неверно указан квартал:`{0}. Должен быть от 1 до 4 включительно.")]
    InvalitQuarter(u8),
}

#[derive(Error, Debug)]
pub enum ParserError {}
