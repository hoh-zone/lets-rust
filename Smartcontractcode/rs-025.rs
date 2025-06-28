pub fn sum(s: &[i32]) -> i32 {
    let mut total = 0;
    for i in s {
        total += i;
    }
    total
}

pub fn reverse(s: &mut [i32]) {
    let n = s.len();
    let k = n / 2;
    for i in 0..k {
        let tmp = s[i];
        s[i] = s[n - 1 - i];
        s[n -1 -i] = tmp;
    }
}

pub fn find(s: &[i32], target: i32) -> Option<usize> {
    let n = s.len();
    if n == 0 {
        return None;
    }
    
    let mut low = 0;
    let mut high = n;
    
    while low < high {
        let mid = (low + high) / 2;
        println!("mid {}", mid);
        if target < s[mid] {
            high = mid;
        } else if s[mid] < target {
            low = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}