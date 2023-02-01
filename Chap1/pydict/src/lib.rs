use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn add_one<'a>(input_dict: &'a PyDict) -> PyResult<&'a PyDict> {
    match input_dict.get_item("input") {
        Some(data) => {
            let input: i32 = data.extract::<i32>().unwrap();
            input_dict.set_item("output", input + 1);
        }
        None => println!("no input found in the dictionary")
    }
    Ok(input_dict)
}



/// A Python module implemented in Rust.
#[pymodule]
fn pydict(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_one, m)?)?;
    Ok(())
}