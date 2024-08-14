/*
Internal types
*/

use crate::renderer::{Circle, Line, Nothing, Polygon, Render};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Indeterminate,
    Undefined,
    Point(Point),
    Triangle(Triangle),
    Angle(Angle),
}

impl Element for Value {
    /// Turn value into a SVG element
    fn to_svg(&self) -> Vec<Box<dyn Render>> {
        match self {
            Value::Point(p) => p.to_svg(),
            Value::Triangle(t) => t.to_svg(),
            Value::Undefined => vec![Box::new(Nothing)],
            _ => vec![Box::new(Polygon { points: vec![] })],
        }
    }
}

pub trait Operation {
    fn box_clone(&self) -> Box<dyn Operation>;
    fn call(&self, args: &[Value]) -> Result<Value, String>;
}

pub trait Element {
    fn to_svg(&self) -> Vec<Box<dyn Render>>;
}

/*
Basic geometric types
*/

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Element for Point {
    /// Turn point into a SVG element
    fn to_svg(&self) -> Vec<Box<dyn Render>> {
        vec![Box::new(Circle {
            center: *self,
            radius: 2.0,
        })]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle {
    pub start: Point,
    pub center: Point,
    pub end: Point,
}

impl Element for Angle {
    /// Turn angle into a SVG element
    fn to_svg(&self) -> Vec<Box<dyn Render>> {
        let first: Line = Line {
            start: self.center,
            end: self.start,
        };
        let second: Line = Line {
            start: self.center,
            end: self.end,
        };
        vec![Box::new(first), Box::new(second)]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Element for Triangle {
    /// Turn triangle into a SVG element
    fn to_svg(&self) -> Vec<Box<dyn Render>> {
        vec![Box::new(Polygon {
            points: vec![self.a, self.b, self.c],
        })]
    }
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
