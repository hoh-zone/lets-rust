pub fn get<F: Fn() -> usize> (f: F) -> usize {
    f()
}

pub fn push<F: FnMut()>(mut f: F) {
    f();
}

pub fn take<F: FnOnce()>(f: F) {
    f();
}