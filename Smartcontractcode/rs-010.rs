#![allow(unused)]

pub struct Circle {
    pub center: (i32, i32),
    pub radius: u32,
}

impl Circle {
    pub fn unit() -> Self {
        Self {
            center: (0,0),
            radius: 1,
        }
    }
}

impl Circle {
    pub fn area(&self) -> f32 {
        let r = self.radius as f32;
        3.14 * r * r
    }
}

impl Circle {
    pub fn shift(&mut self, dx: i32, dy: i32) {
        self.center.0 += dx;
        self.center.1 += dy;
    }
}