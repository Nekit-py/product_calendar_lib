pub mod day;
pub mod parser;
pub mod pc;
pub mod statistic;

use pyo3::prelude::*;
use std::collections::HashMap;

#[pyfunction]
fn prod_cal(year: Option<u16>) -> PyResult<Vec<HashMap<String, String>>> {
    let calendar = pc::get_product_calendar(year).unwrap();
    Ok(calendar.as_vec_hashmap())
}

#[pyfunction]
fn prod_cal_statistic(year: Option<u16>) -> PyResult<HashMap<String, u16>> {
    let calendar = pc::get_product_calendar(year).unwrap();
    Ok(calendar.statistic().as_map())
}

#[pymodule]
fn product_calendar(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(prod_cal, m)?)?;
    m.add_function(wrap_pyfunction!(prod_cal_statistic, m)?)?;
    Ok(())
}
