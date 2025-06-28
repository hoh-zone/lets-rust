pub fn pow(x: u32, n: u32) -> u32 {
    let mut prod = 1;
    for _ in 0..n {
        prod *= x;
    }
    return prod;
}

pub fn sum(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums.iter() {
        total += n;
    }
    total
}

pub fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    
    let mut f0 = 0;
    let mut f1 = 1;
    for _ in 2..=n {
        (f1, f0) = (f1 + f0, f1);
    }
    f1
}