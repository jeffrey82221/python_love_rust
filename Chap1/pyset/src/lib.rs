use pyo3::prelude::*;
use pyo3::types::{PySet, PyList};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn convert_set_to_list(set: &PySet) -> PyResult<Vec<i32>> {
    let mut list: Vec<i32> = Vec::new();
    for item in set.iter(){
        list.push(item.extract::<i32>()?)
    }
    
    Ok(list)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyset(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert_set_to_list, m)?)?;
    Ok(())
}