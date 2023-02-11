use crate::shape::base::Shape;

pub struct ShapeBag {
    pub shapes: Vec<Box<dyn Shape>>,
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