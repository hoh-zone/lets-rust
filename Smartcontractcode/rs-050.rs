pub fn take_and_sum(mut v: Vec<u32>) -> u32 {
    let mut total = 0;
    while let Some(num) = v.pop() {
        total += num;
    }
    total
}