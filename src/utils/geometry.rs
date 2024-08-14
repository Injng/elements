use crate::lang::types::Point;

use std::f64::consts::PI;

/// Function that returns the distance between two points
pub fn distance(first: Point, second: Point) -> f64 {
    ((first.x - second.x).powi(2) + (first.y - second.y).powi(2)).sqrt()
}
