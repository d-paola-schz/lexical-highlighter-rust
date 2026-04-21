mod lexer;
mod html_generator;
mod token;

use lexer::{Lexer};
use html_generator::{html_generator, file_generator};
use std::env;
use std::fs;

// use crate::token::{Token, TokenType};

fn main() -> std::io::Result<()>{
    let mut path = env::current_dir()?;
    path.push("src/input.txt"); 

    let python_code = fs::read_to_string(path)?;

    let mut lexer = Lexer::new(python_code);
    let tokens = lexer.tokenize();
    let html_content = html_generator(tokens);
    file_generator(html_content)?;
    println!("Archivo htmlgenerado correctamente");
    Ok(())
}
