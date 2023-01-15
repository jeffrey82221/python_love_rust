use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn run_class(op: &str, a: usize, b: usize) -> PyResult<usize> {
    Ok((a + b))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn operation(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_class, m)?)?;
    Ok(())
}