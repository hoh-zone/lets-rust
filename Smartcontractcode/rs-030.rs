use std::collections::HashSet;

pub fn remove_dup(vals: Vec<i32>) -> Vec<i32> {
    let mut v = vec![];
    let mut set = HashSet::new();
    
    for x in vals {
        if set.contains(&x) {
            continue;
        }
        set.insert(x);
        v.push(x);
    }
    
    v
}