#[derive(Debug, PartialEq)]
pub struct Rust;

#[derive(Debug, PartialEq)]
pub struct Solidity;

pub trait Compiler: std::fmt::Debug {
    fn compile(&self) -> String;
}

impl Compiler for Rust {
    fn compile(&self) -> String {
        "rustc".to_string()
    }
}

impl Compiler for Solidity {
    fn compile(&self) -> String {
        "solc".to_string()
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidLanguage,
}

pub fn compile(lang: &dyn Compiler) -> String {
    lang.compile()
}

pub fn parse(lang: &str) -> Result<Box<dyn Compiler>, ParseError> {
    match lang {
        "rust" => Ok(Box::new(Rust)),
        "solidity" => Ok(Box::new(Solidity)),
        _ => Err(ParseError::InvalidLanguage),
    }
}