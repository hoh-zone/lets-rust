#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub top: i32,
    pub left: i32,
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Transaction {
    pub hash: [u8; 32],
    pub sender: [u8; 20],
    pub receiver: [u8; 20],
    pub value: u32,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub hash: [u8; 32],
    pub number: u32,
    pub timestamp: u32,
    pub transactions: Vec<Transaction>,
}