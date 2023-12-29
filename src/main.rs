mod expr;
mod lexer;
mod lox;
mod parser;
mod token;

use std::env::{self};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let len = args.len();

    println!("{:?}:{:?}", args, len);

    let mut lox = lox::Lox::new();
    if len > 2 {
        println!("Usage: jlox [script]");
    } else if len == 2 {
        lox.run_file(args.nth(1).unwrap().as_str())
    } else {
        lox.run_prompt()?;
    }

    Ok(())
}
