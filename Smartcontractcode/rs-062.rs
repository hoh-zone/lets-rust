use std::thread;

pub fn par_sum(v: &Vec<u32>) -> u32 {
    let n = v.len();
    let (s0, s1) = thread::scope(|scope| {
        let t0 = scope.spawn(|| v[0..(n / 2)].iter().sum::<u32>());
        let t1 = scope.spawn(|| v[(n / 2)..n].iter().sum::<u32>());
        (t0.join().unwrap(), t1.join().unwrap())
    });
    
    s0 + s1
}