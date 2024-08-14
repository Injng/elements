use crate::lang::types::{Element, Point, Value};

pub trait Render {
    /// Render the element as a SVG string
    fn render(&self) -> String;
    /// Get the bounds of the element
    fn get_bounds(&self) -> (Point, Point);
}

pub struct Svg {
    elements: Vec<Box<dyn Render>>,
}

impl Render for Svg {
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
}

pub struct SvgNothing;

impl Render for SvgNothing {
    fn render(&self) -> String {
        String::new()
    }

    fn get_bounds(&self) -> (Point, Point) {
        (Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 })
    }
}

pub struct SvgPolygon {
    pub points: Vec<Point>,
}

impl Render for SvgPolygon {
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
}

pub struct SvgLine {
    pub start: Point,
    pub end: Point,
}

impl Render for SvgLine {
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
}

pub struct SvgCircle {
    pub center: Point,
    pub radius: f64,
}

impl Render for SvgCircle {
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
}

pub fn render(values: Vec<Value>) -> Result<String, String> {
    let mut elements: Vec<Box<dyn Render>> = Vec::new();
    for value in values {
        let svg_elements: Vec<Box<dyn Render>> = value.to_svg();
        elements.extend(svg_elements);
    }
    let svg = Svg { elements };
    Ok(svg.render())
}
