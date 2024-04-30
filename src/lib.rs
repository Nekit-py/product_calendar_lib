pub mod day;
pub mod parser;
pub mod pc;

use pyo3::prelude::*;
use std::collections::HashMap;

#[pyfunction]
fn prod_cal() -> PyResult<Vec<HashMap<String, String>>> {
    let calendar = pc::get_product_calendar(Some(2024)).unwrap();
    Ok(calendar.as_vec_hasmap())
}

#[pymodule]
fn product_calendar(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(prod_cal, m)?)?;
    Ok(())
}
