pub mod day;
pub mod parser;
pub mod pc;
pub mod statistic;

use pyo3::prelude::*;
use std::collections::HashMap;
use day::Day as RustDay;


#[pyclass]
pub struct Day(RustDay);

#[pymethods]
impl Day {
    #[getter]
    fn weekday(&self) -> String {
        self.0.weekday.to_string()
    }

    #[getter]
    fn day(&self) -> String {
        format!("{}", self.0.day.format("%Y-%m-%d"))
    }

    #[getter]
    fn kind(&self) -> String {
        self.0.kind.to_string()
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
