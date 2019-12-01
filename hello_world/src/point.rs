use std::fmt::{Error, Formatter, Display};
use std::ops::Add;
use std::ops;



#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point { Point { x, y } }
    pub fn diff(&self, other: Point) -> String {
        format!("DIFF({} -- {}): x: {}  y: {}", self, other, self.x - other.x, self.y - other.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{{x: {}, y: {}}}", self.x, self.y)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, r: Point) -> Self::Output {
        Point::new(self.x + r.x, self.y + r.y)
    }
}