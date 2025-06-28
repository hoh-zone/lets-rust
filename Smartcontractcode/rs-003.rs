pub fn vars() -> i32 {
    let x: i32 = -11;
    
    let mut y = 22;
    y += 33;
    x * y
}


fn main() {
    println!("x: {}", vars());
}