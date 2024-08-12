use crate::lang::types::{Operation, Point, Triangle, Value};

/*
Basic arithmetic functions
*/

pub struct FnAdd;
impl Operation for FnAdd {
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

pub struct FnSub;
impl Operation for FnSub {
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

pub struct FnMul;
impl Operation for FnMul {
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

pub struct FnDiv;
impl Operation for FnDiv {
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

pub struct FnNop;
impl Operation for FnNop {
    fn call(&self, _: &[Value]) -> Result<Value, String> {
        Ok(Value::Int(0))
    }
}

/*
Basic geometric functions
*/

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
                Value::Point(p) => points.push(p),
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
    fn call(&self, args: &[Value]) -> Result<Value, String> {
        match self.from_points(args) {
            Ok(triangle) => Ok(triangle),
            _ => Err("Invalid arguments for triangle".to_string()),
        }
    }
}

pub struct FnPoint;
impl Operation for FnPoint {
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
