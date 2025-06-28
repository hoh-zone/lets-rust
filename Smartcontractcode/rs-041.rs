pub trait First<T> {
    fn first(&self) -> Option<&T>;
}

impl<T> First<T> for Vec<T> {
    fn first(&self) -> Option<&T> {
        self.get(0)
    }
}

impl<X, Y,Z> First<X> for (X, Y, Z) {
    fn first(&self) -> Option<&X> {
        Some(&self.0)
    }
}