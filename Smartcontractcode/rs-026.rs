pub fn print(s: &str) {
    println!("{s}");
}

pub fn append(s: &mut String) {
    s.push_str("!");
}

pub fn make() -> String {
    "rust".to_string()
}