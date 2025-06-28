use std::sync::{Arc, Mutex};
use std::thread;

pub fn count() -> u32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut count = c.lock().unwrap();
            *count += 1;
        });
        handles.push(h);
    }
    
    for h in handles {
        h.join().unwrap();
    }
    
    let val = counter.lock().unwrap();
    *val
}

pub fn par_sum(v: Vec<u32>) -> u32 {
    let n = v.len();
    let v0 = Arc::new(v);
    let v1 = Arc::clone(&v0);
    let v2 = Arc::clone(&v1);
    
    let t1 = thread::spawn(move || v1[0..(n / 2)].iter().sum());
    let t2 = thread::spawn(move || v2[(n / 2)..n].iter().sum());
    
    let s1: u32 = t1.join().unwrap();
    let s2: u32 = t2.join().unwrap();
    
    s1 + s2
}