use crate::lang::types::{Element, Point, Value};

pub trait Render {
    fn render(&self) -> String;
}

pub struct Svg {
    elements: Vec<Box<dyn Render>>,
}

impl Render for Svg {
    fn render(&self) -> String {
        let mut elements = String::new();
        for element in &self.elements {
            elements.push_str(&element.render());
        }
        format!(
            "<svg viewBox=\"0 0 300 200\" xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
            elements
        )
    }
}

pub struct Nothing;

impl Render for Nothing {
    fn render(&self) -> String {
        String::new()
    }
}

pub struct Polygon {
    pub points: Vec<Point>,
}

impl Render for Polygon {
    fn render(&self) -> String {
        let mut points = String::new();
        for point in &self.points {
            points.push_str(&format!("{},{} ", point.x, point.y));
        }
        format!(
            "<polygon points=\"{}\" fill=\"none\" stroke=\"black\"/>",
            points
        )
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Render for Circle {
    fn render(&self) -> String {
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\"/>",
            self.center.x, self.center.y, self.radius
        )
    }
}

pub fn render(values: Vec<Value>) -> Result<String, String> {
    let mut elements: Vec<Box<dyn Render>> = Vec::new();
    for value in values {
        elements.push(value.to_svg());
    }
    let svg = Svg { elements };
    Ok(svg.render())
}
