pub mod day;
pub mod parser;
pub mod pc;
pub mod statistic;

use chrono::{Local, NaiveDate, Weekday, Datelike};
use pyo3::prelude::*;
// use std::collections::HashMap;
// use pyo3::types::{PyDict, PyTuple};
use day::{deser::weekday, kind::DayKind, Day as RustDay};


#[pyclass]
pub struct Day(RustDay);

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
            kind
        })
    }

    #[getter]
    fn weekday(&self) -> PyResult<String >{
        Ok(self.0.weekday.to_string())
    }

    #[getter]
    fn day(&self, py: Python<'_>) -> PyObject {
        let date = self.0.day.into_py(py);
        date
    }

    #[getter]
    fn kind(&self) -> PyResult<String >{
        Ok(self.0.kind.to_string())
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
