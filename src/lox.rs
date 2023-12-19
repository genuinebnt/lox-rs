use std::fmt;

use crate::lexer::*;

#[derive(Debug)]
pub enum LoxError {
    IoError(std::io::Error),
    LexError,
}

impl std::error::Error for LoxError {}

impl From<std::io::Error> for LoxError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IoError: {}", e),
            Self::LexError => write!(f, "LexError"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Lox {
    contents: String,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            contents: String::new(),
        }
    }

    pub fn run_file(&mut self, path: &str) {
        self.contents = std::fs::read_to_string(path).unwrap();

        let mut lexer = Lexer::new(&self.contents);

        println!("{:?}", lexer.collect::<Vec<_>>());
    }

    pub fn run_prompt(&self) -> Result<(), LoxError> {
        let mut line = String::new();

        loop {
            std::io::stdin().read_line(&mut line);
            if line == "exit" {
                break;
            }
            let mut lexer = Lexer::new(&line);
            println!("{:?}", lexer.collect::<Vec<_>>());
        }

        Ok(())
    }
}
