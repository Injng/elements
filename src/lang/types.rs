use crate::{
    renderer::{Render, SvgCircle, SvgLabel, SvgLine, SvgNothing, SvgPolygon},
    TOLERANCE,
};

use std::f64::consts::PI;

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
    Circle(Circle),
    Lineseg(Lineseg),
}

impl Element for Value {
    /// Turn value into a SVG element
    fn to_svg(&self) -> Vec<Box<dyn Render>> {
        match self {
            Value::Point(p) => p.to_svg(),
            Value::Triangle(t) => t.to_svg(),
            Value::Angle(a) => a.to_svg(),
            Value::Circle(c) => c.to_svg(),
            Value::String(s) => s.to_svg(),
            Value::Lineseg(l) => l.to_svg(),
            Value::Undefined => vec![Box::new(SvgNothing)],
            _ => vec![Box::new(SvgPolygon { points: vec![] })],
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

/// Implement Element for string labels
impl Element for String {
    fn to_svg(&self) -> Vec<Box<dyn Render>> {
        // extract name and point values from the string
        let mut parts = self.split_whitespace();
        let name = parts.next().unwrap();
        let x = parts.next().unwrap().parse::<f64>().unwrap();
        let y = parts.next().unwrap().parse::<f64>().unwrap();
        let loc = Point { x, y };

        vec![Box::new(SvgLabel {
            text: name.to_string(),
            pt: loc,
            position: None,
        })]
    }
}

/*
Basic geometric types
*/

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lineseg {
    pub start: Point,
    pub end: Point,
}

impl Lineseg {
    /// Return the slope of the lineseg
    pub fn slope(&self) -> f64 {
        (self.end.y - self.start.y) / (self.end.x - self.start.x)
    }

    /// Return the y intercept of the lineseg
    pub fn y_intercept(&self) -> f64 {
        self.start.y - self.slope() * self.start.x
    }
}

impl Element for Lineseg {
    /// Turn lineseg into a SVG element
    fn to_svg(&self) -> Vec<Box<dyn Render>> {
        vec![Box::new(SvgLine {
            start: self.start,
            end: self.end,
        })]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Element for Point {
    /// Turn point into a SVG element
    fn to_svg(&self) -> Vec<Box<dyn Render>> {
        vec![Box::new(SvgCircle {
            center: *self,
            radius: 0.05,
            fill: true,
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
        let first: SvgLine = SvgLine {
            start: self.center,
            end: self.start,
        };
        let second: SvgLine = SvgLine {
            start: self.center,
            end: self.end,
        };
        vec![Box::new(first), Box::new(second)]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Element for Circle {
    /// Turn circle into a SVG element
    fn to_svg(&self) -> Vec<Box<dyn Render>> {
        vec![Box::new(SvgCircle {
            center: self.center,
            radius: self.radius,
            fill: false,
        })]
    }
}

impl Circle {
    /// Create a new circle given a center and radius
    pub fn new(center: Point, radius: f64) -> Result<Self, String> {
        // check for negative radius
        if radius < 0.0 {
            return Err("Radius is negative".to_string());
        }

        // otherwise, return the circle
        Ok(Self { center, radius })
    }

    /// Return a random point on the circle
    pub fn get_point(&self) -> Point {
        let angle = rand::random::<f64>() * 2.0 * PI;
        Point {
            x: self.center.x + self.radius * angle.cos(),
            y: self.center.y + self.radius * angle.sin(),
        }
    }

    /// Check if a point is on the circle
    pub fn is_point_on_circle(&self, point: Point) -> bool {
        let lhs: f64 = (point.x - self.center.x).powi(2) + (point.y - self.center.y).powi(2);
        let rhs: f64 = self.radius.powi(2);
        (lhs - rhs).abs() < TOLERANCE
    }

    /// Return the point on a specified arc from a given angle
    pub fn get_point_on_arc(&self, start: Point, end: Point, deg: f64) -> Result<Point, String> {
        // ensure that the points are on the circle
        if !self.is_point_on_circle(start) || !self.is_point_on_circle(end) {
            return Err("Points are not on the circle".to_string());
        }

        // initialize variables
        let angle = deg.to_radians();
        let center = self.center;
        let radius = self.radius;

        // calculate angles from center to start and end points
        let start_angle = (start.y - center.y).atan2(start.x - center.x);
        let end_angle = (end.y - center.y).atan2(end.x - center.x);

        // normalize angles to be between 0 and 2PI
        let start_angle = if start_angle < 0.0 {
            start_angle + 2.0 * PI
        } else {
            start_angle
        };
        let end_angle = if end_angle < 0.0 {
            end_angle + 2.0 * PI
        } else {
            end_angle
        };

        // always utilize the larger arc
        let mut direction: f64 = 1.0;
        if (end_angle - start_angle).abs() < PI {
            if start_angle < end_angle {
                direction = -1.0;
            }
        } else {
            if start_angle < end_angle {
                direction = -1.0;
            }
        }

        Ok(Point {
            x: center.x + radius * (start_angle + direction * 2.0 * angle).cos(),
            y: center.y + radius * (start_angle + direction * 2.0 * angle).sin(),
        })
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
        vec![Box::new(SvgPolygon {
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

    /// Return the inradius of the triangle
    pub fn inradius(&self) -> f64 {
        // calculate the side lengths
        let a = (self.b.x - self.c.x).hypot(self.b.y - self.c.y);
        let b = (self.a.x - self.c.x).hypot(self.a.y - self.c.y);
        let c = (self.a.x - self.b.x).hypot(self.a.y - self.b.y);

        // calculate the semiperimeter
        let s = (a + b + c) / 2.0;

        // calculate the inradius
        (s * (s - a) * (s - b) * (s - c)).sqrt() / s
    }

    /// Return the incenter of the triangle
    pub fn incenter(&self) -> Point {
        // calculate the side lengths
        let a = (self.b.x - self.c.x).hypot(self.b.y - self.c.y);
        let b = (self.a.x - self.c.x).hypot(self.a.y - self.c.y);
        let c = (self.a.x - self.b.x).hypot(self.a.y - self.b.y);

        // calculate the incenter
        let x = (a * self.a.x + b * self.b.x + c * self.c.x) / (a + b + c);
        let y = (a * self.a.y + b * self.b.y + c * self.c.y) / (a + b + c);

        Point { x, y }
    }

    /// Return the orthocenter of the triangle
    pub fn orthocenter(&self) -> Point {
        // calculate the slopes of the sides
        let m1: f64 = (self.b.y - self.a.y) / (self.b.x - self.a.x);
        let m2: f64 = (self.c.y - self.b.y) / (self.c.x - self.b.x);

        // calculate the perpendicular slopes
        let p1 = -1.0 / m1;
        let p2 = -1.0 / m2;

        // calculate the coordinates of the orthocenter
        let x = (-p1 * self.c.x + p2 * self.a.x + self.c.y - self.a.y) / (p2 - p1);
        let y = p1 * (x - self.c.x) + self.c.y;

        Point { x, y }
    }

    /// Return the centroid of the triangle
    pub fn centroid(&self) -> Point {
        Point {
            x: (self.a.x + self.b.x + self.c.x) / 3.0,
            y: (self.a.y + self.b.y + self.c.y) / 3.0,
        }
    }

    /// Return the circumcenter of the triangle
    pub fn circumcenter(&self) -> Point {
        // calculate the midpoints of the sides
        let m1 = Point {
            x: (self.a.x + self.b.x) / 2.0,
            y: (self.a.y + self.b.y) / 2.0,
        };
        let m2 = Point {
            x: (self.b.x + self.c.x) / 2.0,
            y: (self.b.y + self.c.y) / 2.0,
        };

        // calculate the slopes of the sides
        let s1 = (self.b.y - self.a.y) / (self.b.x - self.a.x);
        let s2 = (self.c.y - self.b.y) / (self.c.x - self.b.x);

        // calculate the perpendicular slopes
        let p1 = -1.0 / s1;
        let p2 = -1.0 / s2;

        // calculate the circumcenter
        let x = (m2.y - m1.y + p1 * m1.x - p2 * m2.x) / (p1 - p2);
        let y = p1 * (x - m1.x) + m1.y;

        Point { x, y }
    }
}
