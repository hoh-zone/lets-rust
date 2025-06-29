pub fn find<T: PartialEq>(s: &[T], x: T) -> Option<usize> {
    let n = s.len();
    
    for i in 0..n {
        if s[i] == x {
            return Some(i);
        }
    }
    
    None
}

pub fn get_or_default<T>(x: Option<T>, val: T) -> T {
    match x {
        Some(v) => v,
        None => val,
    }
}

pub fn zip<A: Copy, B: Copy>(x: &[A], y: &[B]) -> Vec<(A, B)> {
    let mut v = vec![];
    
    let x_len = x.len();
    let y_len = y.len();
    let n = if x_len <= y_len { x_len } else { y_len };
    
    for i in 0..n {
        v.push((x[i], y[i]));
    }
    
    v
}