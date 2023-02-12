use pyo3::prelude::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn set<T: std::cmp::Eq + std::hash::Hash>(input: Vec<T>) -> HashSet<T> {
    input.into_iter().collect()
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Value { //This is some kind of Union type in python.
    Int(i32),
    String(String),
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Int(n) => n.hash(state),
            Value::String(s) => s.hash(state)
        }
    }
}


#[pyfunction]
pub fn main() -> PyResult<()> {
    let input = vec![1, 2, 3, 3, 3];
    println!("input: {:?}", input);
    let output = set(input);
    println!("output: {:?}", output);
    let values = [
        Value::Int(1), 
        Value::Int(2), 
        Value::Int(3), 
        Value::Int(3), 
        Value::String("apple".to_string()),
        Value::String("apple".to_string()),
    ].to_vec();
    println!("multi-type array: {:?}", values);
    let distinct_values = set(values);
    println!("distinct multi-type set: {:?}", distinct_values);
    Ok(())
}

#[pymodule]
fn set_op( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_function( wrap_pyfunction!( main, m )? )?;

    return Ok( () );
}