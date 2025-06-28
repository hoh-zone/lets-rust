pub fn make_add() -> impl Fn(u32,u32) -> u32 {
    |x, y| x + y
}

pub fn make_append_emoji(mut s: String) -> impl FnMut() -> String {
    move || {
        s += "🦀";
        s.clone()
    }
}

pub fn once(mut v: Vec<u32>) -> impl FnOnce() -> Vec<u32> {
    move || {
        v.push(1);
        v
    }
}