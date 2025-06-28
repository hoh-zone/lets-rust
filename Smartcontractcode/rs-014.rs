#[derive(Debug)]
pub struct Color(pub u8, pub u8, pub u8);

pub fn color_to_string(color: Color) -> String {
    match color {
        Color(0,0,0) => "black",
        Color(255,0,0) => "red",
        Color(0,255,0) => "green",
        Color(0,0,255) => "blue",
        Color(255,255,255) => "white",
        _ => "unknown",
    }
    .to_string()
}

pub fn get_or_default(x: Option<u32>, default_val: u32) -> u32 {
    match x {
        Some(i) => i,
        None => default_val,
    }
}

pub fn get_ok_or_default(res: Result<u32, ()>, default_val: u32) -> u32 {
    match res {
        Ok(i) => i,
        Err(_) => default_val,
    }
}