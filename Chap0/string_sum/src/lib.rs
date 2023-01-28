mod rust_add;
use pyo3::prelude::*;
use rust_add::add::add;
use rust_add::add::hello_call;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    let s: String = add(a, b);
    hello_call(s.as_str());
    Ok(s)
}

/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}