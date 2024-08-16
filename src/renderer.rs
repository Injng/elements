use crate::{
    lang::types::{Element, Point, Value},
    utils::geometry::bresenham,
};

use std::any::Any;

pub trait Render {
    /// Render the element as a SVG string
    fn render(&self) -> String;
    /// Get the bounds of the element
    fn get_bounds(&self) -> (Point, Point);
    /// Mark on an array where pixels are
    fn mark_pixels(&self, bitmap: &mut Vec<Vec<bool>>, scale: f64);
    /// Return self for as_any
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Macro to automatically implement as_any for a struct
macro_rules! impl_as_any {
    ($struct_name:ident) => {
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    };
}

pub struct Svg {
    elements: Vec<Box<dyn Render>>,
}

impl Render for Svg {
    impl_as_any!(Svg);
    fn render(&self) -> String {
        // get the SVG string for each element
        let mut elements = String::new();
        for element in &self.elements {
            elements.push_str(&element.render());
        }

        // calculate the appropriate viewBox
        let (min, max): (Point, Point) = self.get_bounds();
        let padding: f64 = 10.0;
        let width: f64 = max.x - min.x + padding;
        let height: f64 = max.y - min.y + padding;
        let min_x: f64 = min.x - padding / 2.0;
        let min_y: f64 = min.y - padding / 2.0;

        format!(
            "<svg viewBox=\"{} {} {} {}\" xmlns=\"http://www.w3.org/2000/svg\">\n{}</svg>",
            min_x, min_y, width, height, elements
        )
    }

    fn get_bounds(&self) -> (Point, Point) {
        let mut min = Point {
            x: f64::INFINITY,
            y: f64::INFINITY,
        };
        let mut max = Point {
            x: f64::NEG_INFINITY,
            y: f64::NEG_INFINITY,
        };
        for element in &self.elements {
            // Make exception for SvgNothing
            if element.render().is_empty() {
                continue;
            }
            let (element_min, element_max) = element.get_bounds();
            if element_min.x < min.x {
                min.x = element_min.x;
            }
            if element_min.y < min.y {
                min.y = element_min.y;
            }
            if element_max.x > max.x {
                max.x = element_max.x;
            }
            if element_max.y > max.y {
                max.y = element_max.y;
            }
        }
        (min, max)
    }

    fn mark_pixels(&self, bitmap: &mut Vec<Vec<bool>>, scale: f64) {
        for element in &self.elements {
            element.mark_pixels(bitmap, scale);
        }
    }
}

impl Svg {
    /// Get the minimum and maximum points of the viewbox
    pub fn get_viewbox(&self) -> (Point, Point) {
        // calculate the appropriate viewBox
        let (min, max): (Point, Point) = self.get_bounds();
        let padding: f64 = 10.0;
        let min_x: f64 = min.x - padding / 2.0;
        let min_y: f64 = min.y - padding / 2.0;
        let width: f64 = max.x - min.x + padding;
        let height: f64 = max.y - min.y + padding;

        // create the points
        (
            Point { x: min_x, y: min_y },
            Point {
                x: min_x + width,
                y: min_y + height,
            },
        )
    }
}

pub struct SvgNothing;

impl Render for SvgNothing {
    impl_as_any!(SvgNothing);
    fn render(&self) -> String {
        String::new()
    }

    fn get_bounds(&self) -> (Point, Point) {
        (Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 })
    }

    fn mark_pixels(&self, _: &mut Vec<Vec<bool>>, _: f64) {
        // Do nothing
    }
}

pub struct SvgLabel {
    pub text: String,
    pub pt: Point,
    pub position: Option<Point>,
}

impl Render for SvgLabel {
    impl_as_any!(SvgLabel);
    fn render(&self) -> String {
        // extract point from option
        let point = match self.position {
            Some(point) => point,
            None => Point { x: 0.0, y: 0.0 },
        };

        format!(
            "\t<text x=\"{}\" y=\"{}\" font-family=\"serif\" font-size=\"0.5\" fill=\"black\">{}</text>\n",
            point.x, point.y, self.text
        )
    }

    fn get_bounds(&self) -> (Point, Point) {
        let point = match self.position {
            Some(point) => point,
            None => Point { x: 0.0, y: 0.0 },
        };
        (point, point)
    }

    fn mark_pixels(&self, _: &mut Vec<Vec<bool>>, _: f64) {
        // Do nothing
    }
}

impl SvgLabel {
    /// Function to set the position
    pub fn set_position(&mut self, position: Point) {
        self.position = Some(position);
    }
}

pub struct SvgPolygon {
    pub points: Vec<Point>,
}

impl Render for SvgPolygon {
    impl_as_any!(SvgPolygon);
    fn render(&self) -> String {
        let mut points = String::new();
        for point in &self.points {
            points.push_str(&format!("{},{} ", point.x, point.y));
        }
        format!(
            "\t<polygon points=\"{}\" fill=\"none\" stroke=\"black\" stroke-width=\"0.02\"/>\n",
            points
        )
    }

    fn get_bounds(&self) -> (Point, Point) {
        let mut min = Point {
            x: f64::INFINITY,
            y: f64::INFINITY,
        };
        let mut max = Point {
            x: f64::NEG_INFINITY,
            y: f64::NEG_INFINITY,
        };
        for point in &self.points {
            if point.x < min.x {
                min.x = point.x;
            }
            if point.y < min.y {
                min.y = point.y;
            }
            if point.x > max.x {
                max.x = point.x;
            }
            if point.y > max.y {
                max.y = point.y;
            }
        }
        (min, max)
    }

    fn mark_pixels(&self, bitmap: &mut Vec<Vec<bool>>, scale: f64) {
        // set height and width of the bitmap
        let height = bitmap.len();
        let width = bitmap[0].len();

        // helper function to mark a single pixel
        let mut mark_pixel = |x: i32, y: i32| {
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                bitmap[y as usize][x as usize] = true;
            }
        };

        // draw lines between consecutive points
        for i in 0..self.points.len() {
            // scale the points
            let start = Point {
                x: self.points[i].x * scale,
                y: self.points[i].y * scale,
            };
            let end = Point {
                x: self.points[(i + 1) % self.points.len()].x * scale,
                y: self.points[(i + 1) % self.points.len()].y * scale,
            };

            // mark the line
            let points: Vec<(i32, i32)> = bresenham(start, end);
            for (x, y) in points {
                mark_pixel(x, y);
            }
        }
    }
}

pub struct SvgLine {
    pub start: Point,
    pub end: Point,
}

impl Render for SvgLine {
    impl_as_any!(SvgLine);
    fn render(&self) -> String {
        format!(
            "\t<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"0.02\"/>\n",
            self.start.x, self.start.y, self.end.x, self.end.y
        )
    }

    fn get_bounds(&self) -> (Point, Point) {
        let min = Point {
            x: self.start.x.min(self.end.x),
            y: self.start.y.min(self.end.y),
        };
        let max = Point {
            x: self.start.x.max(self.end.x),
            y: self.start.y.max(self.end.y),
        };
        (min, max)
    }

    fn mark_pixels(&self, bitmap: &mut Vec<Vec<bool>>, scale: f64) {
        // set height and width of the bitmap
        let height = bitmap.len();
        let width = bitmap[0].len();

        // helper function to mark a single pixel
        let mut mark_pixel = |x: i32, y: i32| {
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                bitmap[y as usize][x as usize] = true;
            }
        };

        // scale start and end points
        let start = Point {
            x: (self.start.x * scale).round(),
            y: (self.start.y * scale).round(),
        };
        let end = Point {
            x: (self.end.x * scale).round(),
            y: (self.end.y * scale).round(),
        };

        // draw line
        let points: Vec<(i32, i32)> = bresenham(start, end);
        for (x, y) in points {
            mark_pixel(x, y);
        }
    }
}

pub struct SvgCircle {
    pub center: Point,
    pub radius: f64,
}

impl Render for SvgCircle {
    impl_as_any!(SvgCircle);
    fn render(&self) -> String {
        format!(
            "\t<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"black\" stroke-width=\"0.02\"/>\n",
            self.center.x, self.center.y, self.radius
        )
    }

    fn get_bounds(&self) -> (Point, Point) {
        let min = Point {
            x: self.center.x - self.radius,
            y: self.center.y - self.radius,
        };
        let max = Point {
            x: self.center.x + self.radius,
            y: self.center.y + self.radius,
        };
        (min, max)
    }

    fn mark_pixels(&self, bitmap: &mut Vec<Vec<bool>>, scale: f64) {
        // set height and width of the bitmap
        let height = bitmap.len();
        let width = bitmap[0].len();

        // helper function to mark a single pixel
        let mut mark_pixel = |x: i32, y: i32| {
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                bitmap[y as usize][x as usize] = true;
            }
        };

        // scale center point
        let center_x: i32 = (self.center.x * scale).round() as i32;
        let center_y: i32 = (self.center.y * scale).round() as i32;

        // draw circle
        let mut x = 0;
        let mut y = (self.radius * scale) as i32;
        let mut d = ((3.0 - 2.0 * self.radius) * scale) as i32;
        while x <= y {
            mark_pixel(center_x + x, center_y + y);
            mark_pixel(center_x + x, center_y - y);
            mark_pixel(center_x - x, center_y + y);
            mark_pixel(center_x - x, center_y - y);
            mark_pixel(center_x + y, center_y + x);
            mark_pixel(center_x + y, center_y - x);
            mark_pixel(center_x - y, center_y + x);
            mark_pixel(center_x - y, center_y - x);
            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }
}

pub fn render(values: Vec<Value>) -> Result<String, String> {
    let mut elements: Vec<Box<dyn Render>> = Vec::new();
    for value in values {
        let svg_elements: Vec<Box<dyn Render>> = value.to_svg();
        elements.extend(svg_elements);
    }
    let mut svg = Svg { elements };

    // mark pixels on bitmap
    let (_, max_point): (Point, Point) = svg.get_viewbox();
    let scale = 10.0;
    let mut bitmap: Vec<Vec<bool>> =
        vec![vec![false; (max_point.x * scale) as usize]; (max_point.y * scale) as usize];
    svg.mark_pixels(&mut bitmap, scale);

    // for each SvgLabel element, figure out best position to put the label
    for element in &mut svg.elements {
        if let Some(label) = element.as_any_mut().downcast_mut::<SvgLabel>() {
            // get initial center position of element to be labelled
            let center_x: f64 = label.pt.x.round();
            let center_y: f64 = label.pt.y.round();

            // define search and label radii and initialize scores
            let search_radius = 5;
            let label_radius = 1;
            let mut best_x = 0;
            let mut best_y = 0;
            let mut best_score = i32::MIN;

            for dy in -search_radius..=search_radius {
                for dx in -search_radius..=search_radius {
                    let x = (center_x * scale).round() as i32 + dx;
                    let y = (center_y * scale).round() as i32 + dy;

                    let mut score: i32 = 0;
                    for ly in (y - label_radius)..(y + label_radius) {
                        for lx in (x - label_radius)..(x + label_radius) {
                            // if a pixel is taken, reduce the score
                            if bitmap[ly as usize][lx as usize] {
                                score -= 1;
                            }

                            // prefer positions closer to the original center
                            score -= (lx - x).abs() + (ly - y).abs();
                        }
                    }

                    if score > best_score {
                        best_score = score;
                        best_x = x;
                        best_y = y;
                    }
                }
            }

            if best_score > i32::MIN {
                label.set_position(Point {
                    x: best_x as f64 / scale,
                    y: best_y as f64 / scale,
                });
            } else {
                // Fallback to original position if no valid position found
                label.set_position(Point {
                    x: center_x,
                    y: center_y,
                });
            }

            // print position
            println!(
                "{}: ({}, {})",
                label.text,
                label.position.unwrap().x,
                label.position.unwrap().y,
            );
        }
    }

    Ok(svg.render())
}
