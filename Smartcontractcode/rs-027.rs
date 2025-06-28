pub fn inc(x: &mut i32) {
    *x += 1;
}

pub fn write(s:&mut String) {
    *s += "!";
}


pub fn add(x: &i32, y: &i32) -> i32 {
    x + y 
}