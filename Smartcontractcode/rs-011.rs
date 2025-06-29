#![allow(unused)]

fn main() {
    pub fn square_of_sum(a: i32, b: i32) -> i32 {
        let c = a + b;
        c * c
    }

    pub fn is_even(a: i32) -> bool {
        (a & 1) == 0
    }

    let a = 1;
    let b = 2;
    let c = square_of_sum(a, b);
    let d = is_even(c);
    println!("c: {}", c);
    println!("d: {}", d);
}