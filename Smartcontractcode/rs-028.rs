pub fn max(v: &Vec<i32>) -> Option<i32> {
    if v.len() == 0 {
        return None;
    }
    
    let mut m = i32::MIN;
    for a in v {
        if *a >= m {
            m = *a;
        }
    }
    
    Some(m)
}

pub fn reverse(v: &Vec<i32>) -> Vec<i32> {
    let mut r = vec![];
    
    let n = v.len();
    for i in 0..n {
        r.push(v[n - 1 - i]);
    }
    r
}