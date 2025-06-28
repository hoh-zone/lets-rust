#[derive(Debug, PartialEq)]
pub enum MathError {
    DivByZero,
    UnderFlow,
}

fn div(x: u32, y: u32) -> Result<u32, MathError> {
    if y == 0 {
        Err(MathError::DivByZero)
    } else {
        Ok(x / y)
    }
}

fn sub(x: u32, y: u32) -> Result<u32, MathError> {
    if x < y {
        Err(MathError::UnderFlow)
    } else {
        Ok(x - y)
    }
}

pub fn f(a: u32, b: u32, c: u32, d: u32) -> Result<u32, MathError> {
    let x = div(a, b)?;
    let y = div(c, d)?;
    sub(x, y)
}