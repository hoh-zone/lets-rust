pub fn str_to_string(s: &str) -> String {
    s.to_string()
}

pub fn string_to_str(s: &String) -> &str {
    &s
}

pub fn add(a: &str, b: &str) -> String {
    format!("{a}{b}")
}

pub fn slice(s: &String, start: usize, len: usize) -> &str {
    &s[start..(start + len)]
}