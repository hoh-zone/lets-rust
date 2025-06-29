#![allow(unused)]

pub fn borrow_1() {
    let s = String::from("rust");
    let s1 = &s;
    let s2 = &s;
}

pub fn borrow_2() {
    let mut s = String::from("rust");
    let s1 = &mut s;
    s1.push_str("!");
    let s2 = &mut s;
    s2.push_str("!");
}

pub fn borrow_3() {
    let mut s = String::from("rust");
    let s2 = &s;
    let s1 = &mut s;
    s1.push_str("!");
}

pub fn borrow_4() {
    let s = String::from("rust");
    let s1 = &s;
    let s2 = &s;
    {
        s;
    }
    
}