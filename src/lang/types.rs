/*
Internal types
*/

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Indeterminate,
    Point(Point),
    Triangle(Triangle),
}

pub trait Operation {
    fn call(&self, args: &[Value]) -> Result<Value, String>;
}

/*
Basic geometric types
*/

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    /// Create a new triangle given three points
    pub fn new(a: Point, b: Point, c: Point) -> Result<Self, String> {
        // check for collinear points
        if (a.x - b.x) * (a.y - c.y) == (a.x - c.x) * (a.y - b.y) {
            return Err("Points are collinear".to_string());
        }

        // otherwise, return the triangle
        Ok(Self { a, b, c })
    }
}
