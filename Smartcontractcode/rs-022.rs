#![allow(unused)]

pub fn owner_1() {
    let s = String::from("rust");
    let s1 = s;
    // let s2 = s;
}

pub fn owner_2() {
    let s = String::from("rust");
    let s1 = s;
    let s2 = s1;
    // let s3 = s;
}

fn take(s: String) {
    println!("{s}");
}

pub fn drop_1() {
    let s = String::from("rust");
    // take(s);
    let s1 = s;
    let s2 = s1;
}

pub fn drop_2() {
    let s = String::from("rust");
    {
        let s1 = s;
        println!("{s1}");
    }
    // let s2 = s;
}