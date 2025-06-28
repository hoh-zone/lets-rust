pub struct Wrap<T: ?Sized > {
    pub value: Box<T>,
}

pub fn wrap<T: ?Sized>(value: Box<T>) -> Wrap<T> {
    Wrap { value }
}