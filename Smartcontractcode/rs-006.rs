pub fn make() -> [u32; 10] {
    let mut arr: [u32; 10] = [0; 10];
    arr[5] = 1;
    arr
}

pub fn first_2(nums: &[i32; 10]) -> &[i32] {
    &nums[..2]
}

pub fn mid_3(nums: &[i32; 10]) -> &[i32] {
    &nums[4..7]
}

pub fn last_2(nums: &[i32; 10]) -> &[i32] {
    &nums[8..]
}