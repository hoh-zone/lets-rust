pub trait Color {
    fn name(&self) -> &str;
}

pub trait Shape {
    fn name(&self) -> &str;
}

pub struct Circle {
    pub color: String,
    pub radius: u32,
}

impl Color for Circle {
    fn name(&self) -> &str {
        &self.color
    }
}

impl Shape for Circle {
    fn name(&self) -> &str {
        "circle"
    }
}

pub fn get_color(circle: &Circle) -> &str {
    Color::name(circle)
}

pub fn get_shape(circle: &Circle) -> &str {
    Shape::name(circle)
}