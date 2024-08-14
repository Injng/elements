use crate::interpreter::is_valid_variable;
use crate::lang::types::Angle;
use crate::lang::types::{Circle, Operation, Point, Triangle, Value};
use crate::utils::geometry::distance;

/// Macro to implement cloning a boxed trait object
macro_rules! clone_impl {
    ($name:ident) => {
        fn box_clone(&self) -> Box<dyn Operation> {
            Box::new(self.clone())
        }
    };
}

/*
Function to set a variable
*/

#[derive(Clone)]
pub struct FnSet;
impl Operation for FnSet {
    clone_impl!(FnSet);
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("setq requires exactly 2 arguments".to_string());
        }
        let var_name = match &args[0] {
            Value::String(s) => s,
            _ => return Err("Invalid variable name".to_string()),
        };
        if !is_valid_variable(var_name) {
            return Err("Invalid variable name".to_string());
        }
        Ok(args[1].clone())
    }
}

/*
Basic arithmetic functions
*/

#[derive(Clone)]
pub struct FnAdd;
impl Operation for FnAdd {
    clone_impl!(FnAdd);
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("Add requires exactly 2 arguments".to_string());
        }
        match (&args[0], &args[1]) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            _ => Err("Invalid types for addition".to_string()),
        }
    }
}

#[derive(Clone)]
pub struct FnSub;
impl Operation for FnSub {
    clone_impl!(FnSub);
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("Sub requires exactly 2 arguments".to_string());
        }
        match (&args[0], &args[1]) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            _ => Err("Invalid types for subtraction".to_string()),
        }
    }
}

#[derive(Clone)]
pub struct FnMul;
impl Operation for FnMul {
    clone_impl!(FnMul);
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("Mul requires exactly 2 arguments".to_string());
        }
        match (&args[0], &args[1]) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            _ => Err("Invalid types for multiplication".to_string()),
        }
    }
}

#[derive(Clone)]
pub struct FnDiv;
impl Operation for FnDiv {
    clone_impl!(FnDiv);
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("Div requires exactly 2 arguments".to_string());
        }
        match (&args[0], &args[1]) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            _ => Err("Invalid types for division".to_string()),
        }
    }
}

#[derive(Clone)]
pub struct FnNop;
impl Operation for FnNop {
    clone_impl!(FnNop);
    fn call(&self, _: &[Value]) -> Result<Value, String> {
        Ok(Value::Int(0))
    }
}

/*
Basic geometric components
*/

#[derive(Clone)]
pub struct FnInscribedAngle;
impl FnInscribedAngle {
    /// Case 1: create an inscribed angle given a circle and an degree value
    fn from_circle_degrees(&self, args: &[Value]) -> Result<Value, String> {
        // check for 2 arguments
        if args.len() < 2 {
            return Err("Inscribed angle requires exactly 2 arguments".to_string());
        }

        // check for circle and degree
        let circle = match &args[0] {
            Value::Circle(c) => c,
            _ => return Err("Invalid types for circle".to_string()),
        };
        let degree = match &args[1] {
            Value::Int(d) => *d,
            _ => return Err("Invalid types for degree".to_string()),
        };

        // check if degree exceeds 180 degrees on the circle
        if degree > 180 {
            return Err("Degree exceeds 180 degrees".to_string());
        }

        // get two random points on the circle to create first line
        let mut start = circle.get_point();
        let mut center = circle.get_point();

        // redo if the points are too close to each other
        while distance(start, center) < circle.radius {
            start = circle.get_point();
            center = circle.get_point();
        }

        // get the end point of the angle, always choosing the larger arc
        let end = match circle.get_point_on_arc(start, center, degree as f64) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        Ok(Value::Angle(Angle { start, center, end }))
    }
}

impl Operation for FnInscribedAngle {
    clone_impl!(FnInscribedAngle);
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        match self.from_circle_degrees(args) {
            Ok(angle) => Ok(angle),
            Err(e) => Err(e),
        }
    }
}

#[derive(Clone)]
pub struct FnAngle;
impl FnAngle {
    /// Case 1: create an angle from three points
    fn from_points(&self, args: &[Value]) -> Result<Value, String> {
        // check for 3 arguments
        if args.len() < 3 {
            return Err("Angle requires exactly 3 arguments".to_string());
        }

        // check for 3 points
        let mut points: Vec<Point> = Vec::new();
        for arg in args {
            match arg {
                Value::Point(p) => points.push(p.clone()),
                _ => return Err("Invalid types for point".to_string()),
            }
        }

        // try creating the angle
        Ok(Value::Angle(Angle {
            start: points[0],
            center: points[1],
            end: points[2],
        }))
    }
}

impl Operation for FnAngle {
    clone_impl!(FnAngle);
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        match self.from_points(args) {
            Ok(angle) => Ok(angle),
            _ => Err("Invalid arguments for angle".to_string()),
        }
    }
}

/*
Basic geometric shapes
*/

#[derive(Clone)]
pub struct FnCircle;
impl FnCircle {
    /// Case 1: create a circle from a point and a radius
    fn from_point_radius(&self, args: &[Value]) -> Result<Value, String> {
        // check for 2 arguments
        if args.len() < 2 {
            return Err("Circle requires exactly 2 arguments".to_string());
        }

        // check for point and radius
        let point = match &args[0] {
            Value::Point(p) => p.clone(),
            _ => return Err("Invalid types for point".to_string()),
        };
        let radius = match &args[1] {
            Value::Int(r) => *r as f64,
            Value::Float(r) => *r,
            _ => return Err("Invalid types for radius".to_string()),
        };

        // try creating the circle
        match Circle::new(point, radius) {
            Ok(circle) => Ok(Value::Circle(circle)),
            Err(e) => Err(e),
        }
    }
}

impl Operation for FnCircle {
    clone_impl!(FnCircle);
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        match self.from_point_radius(args) {
            Ok(circle) => Ok(circle),
            Err(e) => Err(e),
        }
    }
}

#[derive(Clone)]
pub struct FnTriangle;
impl FnTriangle {
    /// Case 1: create a triangle from three points
    fn from_points(&self, args: &[Value]) -> Result<Value, String> {
        // check for 3 arguments
        if args.len() < 3 {
            return Err("Point requires exactly 3 arguments".to_string());
        }

        // check for 3 points
        let mut points: Vec<Point> = Vec::new();
        for arg in args {
            match arg {
                Value::Point(p) => points.push(p.clone()),
                _ => return Err("Invalid types for point".to_string()),
            }
        }

        // try creating the triangle
        match Triangle::new(points[0], points[1], points[2]) {
            Ok(triangle) => Ok(Value::Triangle(triangle)),
            Err(e) => Err(e),
        }
    }
}

impl Operation for FnTriangle {
    clone_impl!(FnTriangle);
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        match self.from_points(args) {
            Ok(triangle) => Ok(triangle),
            _ => Err("Invalid arguments for triangle".to_string()),
        }
    }
}

#[derive(Clone)]
pub struct FnPoint;
impl Operation for FnPoint {
    clone_impl!(FnPoint);
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        // check for 2 arguments
        if args.len() < 2 {
            return Err("Point requires exactly 2 arguments".to_string());
        }

        // try forcing the arguments into floats
        let mut floats = Vec::new();
        for arg in args {
            match arg {
                Value::Int(i) => floats.push(*i as f64),
                Value::Float(f) => floats.push(*f),
                _ => return Err("Invalid types for point".to_string()),
            }
        }

        // return the point
        Ok(Value::Point(Point {
            x: floats[0],
            y: floats[1],
        }))
    }
}
