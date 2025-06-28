pub fn plus_one(v: Vec<u32>) -> Vec<u32> {
    v.iter().map(|x| x + 1).collect()
}

pub fn not_zero(v: Vec<u32>) -> Vec<u32> {
    v.into_iter().filter(|&x| x != 0 ).collect()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub fn zip(xs: Vec<u32>, ys: Vec<u32>) -> Vec<Point> {
    xs.into_iter()
        .zip(ys.into_iter())
        .map(|(x,y)| Point {x, y})
        .collect()
}

pub fn factorial(n: u32) -> u32 {
    (1..=n).fold(1, |z, i| z * i)
}