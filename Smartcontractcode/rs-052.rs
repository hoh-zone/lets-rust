pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

pub fn sub(x: u32, y: u32) -> u32 {
    x - y
}

pub fn op(s: &str) -> Option<fn(u32, u32) -> u32> {
    match s {
        "add" => Some(add),
        "sub" => Some(sub),
        _ => None,
    }
}

pub fn map<T: Copy>(v: Vec<T>, f: fn(T) -> T) -> Vec<T> {
    let mut w = vec![];
    for x in v.iter() {
        w.push(f(*x));
    }
    w
}