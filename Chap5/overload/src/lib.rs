/*
In the std::ops module, you can overload the following operator traits:

Add - Overload the + operator
Sub - Overload the - operator
Mul - Overload the * operator
Div - Overload the / operator
Rem - Overload the % operator
Neg - Overload the unary - operator
Not - Overload the unary ! operator
BitAnd - Overload the & operator
BitOr - Overload the | operator
BitXor - Overload the ^ operator
Shl - Overload the << operator
Shr - Overload the >> operator
Index - Overload the indexing operator []
Deref - Overload the dereference operator *
AddAssign - Overload the compound assignment operator +=
SubAssign - Overload the compound assignment operator -=
MulAssign - Overload the compound assignment operator *=
DivAssign - Overload the compound assignment operator /=
RemAssign - Overload the compound assignment operator %=
BitAndAssign - Overload the compound assignment operator &=
BitOrAssign - Overload the compound assignment operator |=
BitXorAssign - Overload the compound assignment operator ^=
ShlAssign - Overload the compound assignment operator <<=
ShrAssign - Overload the compound assignment operator >>=
*/
use pyo3::prelude::*;

use std::ops::{Add, Mul, Sub, Neg, BitOr, AddAssign};

trait Shape {
    fn print(&self);
}

impl Shape for Point {
    fn print(&self) {
        println!("This is Point {}, {}", self.x, self.y)
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, scalar: i32) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point { x: -self.x, y: -self.y }
    }
}

impl Point {
    fn dot(self, other: Point) -> i32 {
        self.x * other.x + self.y * other.y
    }
}

impl BitOr for Point {
    type Output = Vec<Point>;

    fn bitor(self, other: Point) -> Vec<Point> {
        vec![self, other]
    }
}



#[pyfunction]
pub fn main() -> PyResult<()> {
    let p1 = Point {x: 1, y: 1};
    let p2 = Point {x: 2, y: 2};
    let r = p1 + p2;
    r.print();
    (r * 2).print();
    let p1 = Point {x: 1, y: 1};
    let p2 = Point {x: 2, y: 2};
    (p1 - p2).print();
    let p1 = Point {x: 1, y: 1};
    let p2 = Point {x: 2, y: 2};
    println!("Inner prod between p1 and p2: {}", (p1.dot(p2)));
    let p1 = Point {x: 1, y: 2};
    (-p1).print();
    let p1 = Point {x: 1, y: 1};
    let p2 = Point {x: 2, y: 2};
    let c = &p1 + &p2;
    println!("{:?}", c);
    p1.print();
    p2.print();
    let mut p1 = Point {x: 10, y: 10};
    let p2 = Point {x: 1, y: 1};
    p1 += p2;
    p1.print();
    Ok(())
}

#[pymodule]
fn overload( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_function( wrap_pyfunction!( main, m )? )?;

    return Ok( () );
}