// TODO: [ ] Enable override of __add__ __sum__ ... 
use pyo3::prelude::*;

trait Shape {
    fn area(&self) -> f64;
    fn print(&self);
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn print(&self) {
        println!("Rectangle {} X {}", self.width, self.height)
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn print(&self) {
        println!("Circle of Size {}", self.area())
    }
}

#[pyfunction]
pub fn parse() -> PyResult<()> {
    let c = Circle {radius: 10.0};
    c.print();
    let r = Rectangle {width: 10.0, height: 20.0};
    r.print();
    Ok(())
}

#[pymodule]
fn inherit( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_function( wrap_pyfunction!( parse, m )? )?;

    return Ok( () );
}