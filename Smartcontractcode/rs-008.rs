#[derive(Debug, PartialEq)]
pub enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

pub fn move_up(y: u32) -> Move {
    Move::Up(y)
}

pub fn move_down(y: u32) -> Move {
    Move::Down(y)
}