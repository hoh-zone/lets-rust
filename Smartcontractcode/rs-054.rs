pub fn borrow(s: &String) -> String {
    let f = || format!("hello {s}");
    f()
}

pub fn borrow_mut(s: &mut String) {
    let mut f = || s.push_str(" world");
    f()
}

pub fn take(s: String) -> String {
    let f = move || s;
    f()
}