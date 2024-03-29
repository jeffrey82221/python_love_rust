use pyo3::prelude::*;

mod shape;

use shape::{Circle, Rectangle, ShapeBag, Shape};

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