// TODO: [ ] Enable override of __add__ __sum__ ... 
use pyo3::prelude::*;
use std::ops::Add;

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

struct ShapeBag {
    shapes: Vec<Box<dyn Shape>>,
}

impl Shape for ShapeBag {
    fn area(&self) -> f64 {
        let mut i: f64 = 0.0;
        for s in &self.shapes {
            i += s.area();
        }
        i
    }
    fn print(&self) {
        println!("ShapeBag contains:");
        for s in &self.shapes {
            s.print();
        }
    }

}


#[pyfunction]
pub fn parse() -> PyResult<()> {
    let c = Circle {radius: 10.0};
    c.print();
    let r = Rectangle {width: 10.0, height: 20.0};
    r.print();
    let b = ShapeBag { shapes: vec![Box::new(r), Box::new(c)] };
    b.print();
    Ok(())
}

#[pymodule]
fn inherit( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_function( wrap_pyfunction!( parse, m )? )?;

    return Ok( () );
}