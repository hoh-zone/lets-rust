const PI: f64 = 3.14;

pub fn calc_area(r: f64) -> f64 {
    PI * r * r
}

fn main() {
    println!("area: {}", calc_area(10.0));
}