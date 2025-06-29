#[derive(Debug)]
pub struct Rectangle<T> {
    pub top: T,
    pub left: T,
    pub width: T,
    pub height: T,
}

impl<T> Rectangle<T> {
    pub fn resize(&mut self, width: T, height: T) {
        self.width = width;
        self.height = height;
    }
}