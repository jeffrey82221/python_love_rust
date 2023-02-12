use pyo3::prelude::*;
use serde_json::Value;

#[pyfunction]
fn parse(json_string: &str) {
    let value: Value = match serde_json::from_str(json_string) {
        Ok(value) => value,
        Err(error) => {
            println!("Failed to parse JSON string: {}", error);
            return;
        }
    };
    println!("{:?}", value);
}

#[pymodule]
fn json_process( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_function( wrap_pyfunction!( parse, m )? )?;
    return Ok( () );
}