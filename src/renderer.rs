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
            "<svg viewBox=\"0 0 50 50\" xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
            elements
        )
    }
}

pub struct SvgNothing;

impl Render for SvgNothing {
    fn render(&self) -> String {
        String::new()
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
            "<polygon points=\"{}\" fill=\"none\" stroke=\"black\" stroke-width=\"0.02\"/>",
            points
        )
    }
}

pub struct SvgLine {
    pub start: Point,
    pub end: Point,
}

impl Render for SvgLine {
    fn render(&self) -> String {
        format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"0.02\"/>",
            self.start.x, self.start.y, self.end.x, self.end.y
        )
    }
}

pub struct SvgCircle {
    pub center: Point,
    pub radius: f64,
}

impl Render for SvgCircle {
    fn render(&self) -> String {
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"black\" stroke-width=\"0.02\"/>",
            self.center.x, self.center.y, self.radius
        )
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
