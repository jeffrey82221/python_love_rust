// Goal: 
// Make the complex object computation fast. 
// - The complex object computation should be in the Rust domain (implemented with pure Rust objects). 
// - A `reduce` method operate on the Rust domain (input: json str list, output: Object str)
// - The Object str can be transformed to Python object in Python domain. 
// - When doing `|` operation in Python domain, `or` in Rust domain should be called.  
// TODO:
// 1. [ ] build pure rust objects (RustPoint)
// 2. [ ] Let Point pyclass takes RustPoint as variable. 
// 3. [ ] Let RustPoint be able to be converted to a str ("Point(x, y)")
// 4. [ ] Implement methods on RustPoint and call them from the Python Point method. 
use pyo3::prelude::*;
use pyo3::exceptions;
#[derive(Clone)]
#[pyclass]
struct Point {
    #[pyo3(get)]
    x: i32,
    #[pyo3(get)]
    y: i32,
}

#[pymethods]
impl Point {
    #[new]
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
    fn __add__(&self, o: &PyAny) -> PyResult<Point> {
        if let Ok(other_point) = o.extract::<Point>() {
            Ok(Point::new(self.x + other_point.x, self.y + other_point.y))
        } else if let Ok(other_point2) = o.extract::<Point>() {
            Ok(Point::new(self.x + other_point2.x, self.y + other_point2.y))
        } else {
            Err(exceptions::PyTypeError::new_err("Unsupported type"))
        }
    }
}

#[derive(Clone)]
#[pyclass]
struct Point2 {
    #[pyo3(get)]
    x: i32,
    #[pyo3(get)]
    y: i32,
}

#[pymethods]
impl Point2 {
    #[new]
    fn new(x: i32, y: i32) -> Self {
        Point2 { x, y }
    }
}



#[pymodule]
fn rust_objs( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_class::<Point2>()?;
    return Ok( () );
}