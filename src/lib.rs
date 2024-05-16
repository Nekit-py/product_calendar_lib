pub mod day;
pub mod parser;
pub mod pc;
pub mod statistic;
use std::str::FromStr;

use chrono::{Datelike, Local, NaiveDate, Weekday};
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
// use std::collections::HashMap;
// use pyo3::types::{PyDict, PyTuple};
use day::{deser::weekday, kind::DayKind, Day as RustDay};

#[pyclass]
pub struct Day(RustDay);

//TODO Переработать RustDay таким образом, чтобы принмал только дату,
// остальные поля сам рассчитывал
#[pymethods]
impl Day {
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

    fn repr(&self) -> PyResult<String> {
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
        let date = self.0.day.into_py(py);
        date
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
