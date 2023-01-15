use pyo3::prelude::*;
use either::*;

struct Operation {
    op: String
}
impl Operation {
    fn add(a: usize, b: usize) -> usize {
        return a + b;
    }
    fn mul(a: usize, b: usize) -> usize {
        return a * b;
    }
    fn sub(a: usize, b: usize) -> usize {
        return a - b;
    }
    fn div(a: usize, b: usize) -> f64 {
        return (a as f64) / (b as f64)
    }
    fn run(&self, a: usize, b: usize) -> Either<usize, f64> {
        if self.op == "add" {
            return Operation::add(a, b);
        } else if self.op == "mul" {
            return Operation::mul(a, b);
        } else if self.op == "sub" {
            return Operation::sub(a, b);
        } else {
            return Operation::div(a, b);
        }
    }

}
/// Formats the sum of two numbers as string.
#[pyfunction]
fn run_class(input_op: &str, a: usize, b: usize) -> usize {
    let op = Operation {op: input_op.to_string()};
    return op.run(a, b);
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn operation(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_class, m)?)?;
    Ok(())
}