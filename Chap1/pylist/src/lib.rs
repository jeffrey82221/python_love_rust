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

#[pyfunction]
fn add_one_inplace<'a>(input_list: &'a PyList) -> PyResult<()> {
    for (index, item) in input_list.iter().enumerate() {
        let item = item.extract::<i32>()?;
        let new_item = item + 1;
        input_list.set_item(index, new_item)?;
    }
    Ok(())
}
/// A Python module implemented in Rust.
#[pymodule]
fn pylist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_one, m)?)?;
    m.add_function(wrap_pyfunction!(add_one_inplace, m)?)?;
    Ok(())
}