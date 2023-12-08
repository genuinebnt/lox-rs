use crate::token::*;

pub enum ScannerError {
    ScanError(String),
}

#[derive(Debug)]
pub struct Scanner {
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner { tokens: vec![] }
    }

    pub fn scan_tokens(&mut self, contents: &str) -> Result<(), ScannerError::ScanError> {}
}
