use crate::shape::base::Shape;

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
