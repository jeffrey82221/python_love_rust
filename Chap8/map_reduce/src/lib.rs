use pyo3::prelude::*;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use num_cpus;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Value {
    Int(i32)
}


#[pyfunction]
fn main() {
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_cpus::get() * 2)
        .build()
        .unwrap();
    let values = vec![Value::Int(1), Value::Int(2), Value::Int(3)];

    let reduced = pool.install(|| {
        values
            .into_par_iter()
            .map(|value| match value {
                Value::Int(x) => Value::Int(-x)
            })
            .reduce(|| Value::Int(0), |acc, value| match (acc, value) {
                (Value::Int(x), Value::Int(y)) => Value::Int(x + y)
            })
    });
    println!("{:?}", reduced);
}

#[pymodule]
fn map_reduce( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_function( wrap_pyfunction!( main, m )? )?;
    return Ok( () );
}