use std::collections::HashMap;

pub fn key_vals(map: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut kvs = vec![];
    
    for (k,v) in map {
        kvs.push((k,v));
    }
    
    kvs
}

pub fn repeat(v: Vec<u32>, n: u32) -> Vec<u32> {
    let mut w = vec![];
    
    for _ in 0..n {
        for u in v.iter() {
            w.push(*u);
        }
    }
    w
}

pub fn mul_by_2(mut v: Vec<u32>) -> Vec<u32> {
    for u in v.iter_mut() {
        *u *= 2;
    }
    
    v
}