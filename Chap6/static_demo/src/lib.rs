use pyo3::prelude::*;

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn euclidean_distance(p1: &Point, p2: &Point) -> f32 {
        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[pyfunction]
pub fn main() -> PyResult<()> {
    let p1 = Point {x: 1.0, y: 1.0};
    let p2 = Point {x: 2.0, y: 2.0};
    let dis = Point::euclidean_distance(&p1, &p2);
    println!("distance between p1 and p2 is {}", dis);
    Ok(())
}

#[pymodule]
fn static_demo( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_function( wrap_pyfunction!( main, m )? )?;

    return Ok( () );
}