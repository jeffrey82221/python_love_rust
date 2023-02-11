use crate::base::Shape;

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn print(&self) {
        println!("Rectangle {} X {}", self.width, self.height)
    }
}

#[derive(Copy, Clone)]
pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn print(&self) {
        println!("Circle of Size {}", self.area())
    }
}

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