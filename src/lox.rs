use crate::lexer::*;

pub enum LoxError {
    IoError(std::io::Error),
    LexError,
}

impl From<std::io::Error> for LoxError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

#[derive(Debug, Default)]
pub struct Lox {
    contents: String,
}

impl Lox {
    pub fn run_file(&mut self, path: &str) -> Result<(), LoxError> {
        self.contents = std::fs::read_to_string(path)?;

        self.run();
        Ok(())
    }

    fn run(&self) -> Result<(), LoxError> {
        let mut lexer = Lexer::new(&self.contents);

        Ok(())
    }
}

