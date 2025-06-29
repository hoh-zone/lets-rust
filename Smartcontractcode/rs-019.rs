#[derive(Debug, PartialEq)]
pub enum MathError {
    DivByZero,
}

fn div(x: u32, y: u32) -> Result<u32, MathError> {
    if y == 0 {
        Err(MathError::DivByZero)
    } else {
        Ok(x / y)
    }
}

pub fn div_unwrap(x: u32, y: u32) -> u32 {
    div(x, y).unwrap()
}

pub fn div_expect(x: u32, y: u32) -> u32 {
    div(x, y).expect("div error")
}