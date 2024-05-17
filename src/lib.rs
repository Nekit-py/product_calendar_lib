pub mod day;
pub mod errors;
pub mod parser;
pub mod pc;
pub mod statistic;
use std::str::FromStr;
use std::usize;

use chrono::{Datelike, Local, NaiveDate, Weekday};
use day::{kind::DayKind, Day as RustDay};
use pc::{get_product_calendar, ProductCalendar as RustProductCalendar};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{IntoPyDict, PyDict};
use statistic::Statistic as RustStatistic;

#[pyclass]
pub struct ProductCalendar(RustProductCalendar);

#[pymethods]
impl ProductCalendar {
    #[new]
    #[pyo3(signature=(year=None))]
    fn new(year: Option<u16>) -> PyResult<Self> {
        match get_product_calendar(year) {
            Ok(rpc) => Ok(Self(rpc)),
            Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
        }
    }

    fn after_nth_weeks(&self, date: NaiveDate, weeks: usize) -> PyResult<Day> {
        match self.0.after_nth_weeks(date, weeks) {
            Ok(d) => Ok(Day(d)),
            Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
        }
    }

    fn period_by_number_of_days(&self, date: NaiveDate, days: usize) -> PyResult<Self> {
        match self.0.period_by_number_of_days(date, days) {
            Ok(rpc) => Ok(Self(rpc)),
            Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
        }
    }

    fn period_by_number_of_work_days(&self, date: NaiveDate, work_days: usize) -> PyResult<Self> {
        match self.0.period_by_number_of_work_days(date, work_days) {
            Ok(rpc) => Ok(Self(rpc)),
            Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
        }
    }

    fn period_slice(&self, start: NaiveDate, end: NaiveDate) -> PyResult<Self> {
        match self.0.period_slice(start, end) {
            Ok(rpc) => Ok(Self(rpc)),
            Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
        }
    }

    fn extract_dates_in_quarter(&self, quarter: u8) -> PyResult<Self> {
        match self.0.extract_dates_in_quarter(quarter) {
            Ok(calendar) => Ok(Self(calendar)),
            Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
        }
    }

    fn statistic(&self) -> PyResult<Statistic> {
        Ok(Statistic(self.0.statistic()))
    }

    fn total_days(&self) -> PyResult<usize> {
        Ok(self.0.total_days())
    }

    fn next_work_day(&self, cur_day: NaiveDate) -> PyResult<Day> {
        match self.0.next_work_day(cur_day) {
            Ok(d) => Ok(Day(d)),
            Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
        }
    }

    fn by_kind(&self, kind: &str) -> PyResult<Self> {
        let kind = DayKind::from_str(kind).unwrap();
        Ok(Self(self.0.by_kind(kind)))
    }

    fn all_days(&self) -> PyResult<Vec<Day>> {
        Ok(self.0.calendar.iter().map(|d| Day(d.clone())).collect())
    }
}

#[pyclass]
pub struct Statistic(RustStatistic);

#[pymethods]
impl Statistic {
    #[new]
    #[pyo3(signature=(holidays=0, work_days=0, weekends=0, preholidays=0))]
    fn new(holidays: u16, work_days: u16, weekends: u16, preholidays: u16) -> Self {
        Self(RustStatistic {
            holidays,
            weekends,
            work_days,
            preholidays,
        })
    }

    fn work_hours(&self) -> PyResult<u16> {
        Ok(self.0.work_hours())
    }

    fn rest_days(&self) -> PyResult<u16> {
        Ok(self.0.rest_days())
    }

    fn as_dict<'py>(&self, py: Python<'py>) -> Bound<'py, PyDict> {
        self.0.as_map().into_py_dict_bound(py)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.0))
    }

    #[getter]
    fn weekends(&self) -> PyResult<u16> {
        Ok(self.0.weekends)
    }

    #[setter]
    fn set_weekends(&mut self, val: u16) -> PyResult<()> {
        self.0.weekends = val;
        Ok(())
    }

    #[getter]
    fn holidays(&self) -> PyResult<u16> {
        Ok(self.0.holidays)
    }

    #[setter]
    fn set_holidays(&mut self, val: u16) -> PyResult<()> {
        self.0.holidays = val;
        Ok(())
    }

    #[getter]
    fn work_days(&self) -> PyResult<u16> {
        Ok(self.0.work_days)
    }

    #[setter]
    fn set_work_days(&mut self, val: u16) -> PyResult<()> {
        self.0.work_days = val;
        Ok(())
    }

    #[getter]
    fn preholidays(&self) -> PyResult<u16> {
        Ok(self.0.preholidays)
    }

    #[setter]
    fn set_preholidays(&mut self, val: u16) -> PyResult<()> {
        self.0.preholidays = val;
        Ok(())
    }
}

#[pyclass]
pub struct Day(RustDay);

//TODO Переработать RustDay таким образом, чтобы принмал только дату,
// остальные поля сам рассчитывал
#[pymethods]
impl Day {
    /// new(day, /)
    /// --
    ///
    /// Эта функция инициализирует класс Day.
    /// Принимает опционально объект datetime.date
    #[new]
    #[pyo3(signature=(day=None))]
    fn new(day: Option<NaiveDate>) -> Self {
        let today = if let Some(d) = day {
            d
        } else {
            Local::now().date_naive()
        };

        let weekday = today.weekday();
        let kind = match weekday {
            Weekday::Sat | Weekday::Sun => DayKind::Weekend,
            _ => DayKind::Work,
        };

        Self(RustDay {
            day: today,
            weekday,
            kind,
        })
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self.0 == other.0),
            _ => Ok(false),
        }
    }

    #[getter]
    fn weekday(&self) -> PyResult<String> {
        Ok(self.0.weekday.to_string())
    }

    #[getter]
    fn day(&self, py: Python<'_>) -> PyObject {
        self.0.day.into_py(py)
    }

    #[getter]
    fn kind(&self) -> PyResult<String> {
        Ok(self.0.kind.to_string())
    }

    #[setter]
    fn set_kind(&mut self, val: &str) -> PyResult<()> {
        self.0.kind = DayKind::from_str(val).unwrap();
        Ok(())
    }
}

#[pymodule]
fn product_calendar(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(prod_cal, m)?)?;
    // m.add_function(wrap_pyfunction!(prod_cal_statistic, m)?)?;
    m.add_class::<ProductCalendar>()?;
    m.add_class::<Statistic>()?;
    m.add_class::<Day>()?;
    Ok(())
}

// #[pyfunction]
// fn prod_cal(year: Option<u16>) -> PyResult<Vec<HashMap<String, String>>> {
//     let calendar = pc::get_product_calendar(year).unwrap();
//     Ok(calendar.as_vec_hashmap())
// }

// #[pyfunction]
// fn prod_cal_statistic(year: Option<u16>) -> PyResult<HashMap<String, u16>> {
//     let calendar = pc::get_product_calendar(year).unwrap();
//     Ok(calendar.statistic().as_map())
// }

// #[pymodule]
// fn product_calendar(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(prod_cal, m)?)?;
//     m.add_function(wrap_pyfunction!(prod_cal_statistic, m)?)?;
//     Ok(())
// }
