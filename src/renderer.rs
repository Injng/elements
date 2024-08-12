use crate::lang::types::Point;

pub trait Render {
    fn render(&self) -> String;
}

pub struct Svg<T: Render> {
    elements: Vec<T>,
}

impl Render for Svg<T> {
    fn render(&self) -> String {
        let mut elements = String::new();
        for element in &self.elements {
            elements.push_str(&element.render());
        }
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
            elements
        )
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Render for Polygon {
    fn render(&self) -> String {
        let mut points = String::new();
        for point in &self.points {
            points.push_str(&format!("{},{} ", point.x, point.y));
        }
        format!("<polygon points=\"{}\" />", points)
    }
}

pub struct Circle {
    center: Point,
    radius: f64,
}

impl Render for Circle {
    fn render(&self) -> String {
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" />",
            self.center.x, self.center.y, self.radius
        )
    }
}
