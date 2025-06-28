pub fn return_tuple() -> (i32, i32, bool) {
    (1, -1, true)
}

pub fn swap(t: (i32, i32)) -> (i32, i32) {
    (t.1 , t.0)
}