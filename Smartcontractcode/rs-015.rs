pub fn get_or_default(x: Option<u32>, default_val: u32) -> u32 {
    if let Some(i) = x {
        i
    } else {
        default_val
    }
}