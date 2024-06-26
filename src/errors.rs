use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProductCalendarError {
    #[error("Данные на `{0}` год недоступны.")]
    InvalidYear(String),
    #[error("Дата `{0}` находится вне диапазона текущего производственного календаря.")]
    DateOutOfRange(String),
    #[error("Количество дней: `{0}` превышает максимально допустимое значение")]
    ExceedMaxDaysError(usize),
    #[error("Неверно указан квартал:`{0}. Должен быть от 1 до 4 включительно.")]
    InvalidQuarter(u8),
}
