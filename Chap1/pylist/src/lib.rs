use pyo3::prelude::*;
use pyo3::types::PyList;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn add_one<'a>(input_list: &'a PyList) -> PyResult<Vec<i32>> {
    let result: Vec<i32> =
        input_list.extract::<Vec<i32>>().unwrap().iter().map(
            |x| x + 1
        ).collect();
    
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pylist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_one, m)?)?;
    Ok(())
}