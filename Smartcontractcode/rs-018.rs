pub fn div(x: u32, y: u32) -> Result<u32, String> {
    if y == 0 {
        Err("div by 0".to_string())
    } else {
        Ok(x /y)
    }
}

pub fn find_index(nums: &[i32], x: i32) -> Option<usize> {
    for i in 0..nums.len() {
        if nums[i] == x {
            return Some(i);
        }
    }
    return None;
}