mod class_module;
use pyo3::prelude::{pymodule, PyResult, PyModule, Python};
use class_module::logic_unit::LogicUnit;

/// A Python module implemented in Rust.
#[pymodule]
fn logic_unit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LogicUnit>()?;
    Ok(())
}