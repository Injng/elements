use crate::lang::types::Point;

/// Function that returns the midpoint between two points
pub fn midpoint(first: Point, second: Point) -> Point {
    Point {
        x: (first.x + second.x) / 2.0,
        y: (first.y + second.y) / 2.0,
    }
}

/// Function that returns the distance between two points
pub fn distance(first: Point, second: Point) -> f64 {
    ((first.x - second.x).powi(2) + (first.y - second.y).powi(2)).sqrt()
}

/// Function that uses Bresenham's line algorithm to return a vector of coordinates
pub fn bresenham(start: Point, end: Point) -> Vec<(i32, i32)> {
    // set initial and end points
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    // calculate line function and error term
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut result = Vec::new();

    // iterate through points, following the line within the error term
    loop {
        result.push((x0, y0));
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }

    result
}
