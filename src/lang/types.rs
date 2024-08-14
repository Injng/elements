use crate::renderer::{Render, SvgCircle, SvgLine, SvgNothing, SvgPolygon};

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
}

impl Element for Value {
    /// Turn value into a SVG element
    fn to_svg(&self) -> Vec<Box<dyn Render>> {
        match self {
            Value::Point(p) => p.to_svg(),
            Value::Triangle(t) => t.to_svg(),
            Value::Angle(a) => a.to_svg(),
            Value::Circle(c) => c.to_svg(),
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
        vec![Box::new(SvgCircle {
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
        (point.x - self.center.x).powi(2) + (point.y - self.center.y).powi(2) == self.radius.powi(2)
    }

    /// Return the point on a specified arc from a given angle
    pub fn get_point_on_arc(&self, start: Point, end: Point, deg: f64) -> Result<Point, String> {
        // ensure that the points are on the circle
        /*
        if !self.is_point_on_circle(start) || !self.is_point_on_circle(end) {
            return Err("Points are not on the circle".to_string());
        }
        */
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
}
