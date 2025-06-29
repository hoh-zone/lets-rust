fn max(a: usize, b: usize) -> usize {
    if a >= b {
        a
    } else {
        b
    }
}

pub fn max_len(a: &String, b: &String) -> usize {
    max(a.len(), b.len())
}



pub fn add(a: &mut String, b: &String) {
    a.push_str(b);
}