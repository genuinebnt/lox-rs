use std::{fmt, io::Write};

use crate::{
    interpreter::{Interpreter, RuntimeError},
    lexer::*,
    parser::{Parser, ParserError},
};

#[derive(Debug)]
pub enum LoxError {
    IoError(std::io::Error),
    ParserError(String),
    RuntimeError(String),
}

impl std::error::Error for LoxError {}

impl From<std::io::Error> for LoxError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<ParserError<'_>> for LoxError {
    fn from(value: ParserError) -> Self {
        match value {
            ParserError::Eof => Self::ParserError("Reached enf of file".to_string()),
            ParserError::UnexpectedToken(e) => Self::ParserError(format!("Unexpected token {}", e)),
            ParserError::UnexpectedBinaryOp(e) => {
                Self::ParserError(format!("Unexpected binary op {}", e))
            }
        }
    }
}

impl From<RuntimeError> for LoxError {
    fn from(value: RuntimeError) -> Self {
        match value {
            RuntimeError::DivisionByZero => {
                Self::RuntimeError("Runtime error division by zero".to_string())
            }
            RuntimeError::InvalidOperand => Self::RuntimeError("Invalid operand".to_string()),
            RuntimeError::InvalidOperation => Self::RuntimeError("Invalid operation".to_string()),
        }
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IoError: {}", e),
            Self::ParserError(e) => write!(f, "Parser Error: {}", e),
            Self::RuntimeError(e) => write!(f, "Runtime Error: {}", e),
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

    pub fn run_file(&mut self, path: &str) -> Result<(), LoxError> {
        self.contents = std::fs::read_to_string(path).unwrap();

        let lexer = Lexer::new(&self.contents);

        let mut parser = Parser::new(lexer);

        let expr = parser.parse()?;

        let mut interpreter = Interpreter::new();

        println!("{:?}", interpreter.interpret(expr)?);

        Ok(())
    }

    pub fn run_prompt(&self) -> Result<(), LoxError> {
        let mut line = String::new();

        loop {
            std::io::stdout().flush().unwrap();
            print!(">");
            std::io::stdin().read_line(&mut line).unwrap();
            if line == "exit" {
                break;
            }
            let lexer = Lexer::new(&line);

            let mut parser = Parser::new(lexer);
            println!("{}", parser.parse()?);
        }

        Ok(())
    }
}
