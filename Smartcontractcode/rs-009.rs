#[derive(Debug)]
pub struct Rectangle {
    pub top: i32,
    pub left: i32,
    pub height: u32,
    pub width: u32,
}

pub fn make(top: i32, left: i32, height: u32, width: u32) -> Rectangle {
    Rectangle {
        top,
        left,
        height,
        width,
    }
}

pub fn move_to(rect: &mut Rectangle, top: i32, left: i32) {
    rect.top = top;
    rect.left = left;
}

pub fn grow(rect: &mut Rectangle, scale_factor: u32) {
    rect.height *= scale_factor;
    rect.width *= scale_factor;
}

pub fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}