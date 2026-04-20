mod lexer;
mod html_generator;
mod token;

use lexer::{Lexer};
use html_generator::{htmlGenerator, fileGenerator};

use crate::token::{Token, TokenType};

fn main() -> std::io::Result<()>{
    let python_code = String::from("def sum(x,y):\n return mama\n\t#pruebas de lectura");
    let mut lexer = Lexer::new(python_code);
    let tokens = lexer.tokenize();
    let html_content = htmlGenerator(tokens);
    fileGenerator(html_content)?;
    println!("Archivo htmlgenerado correctamente");
    Ok(())
}
