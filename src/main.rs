mod lexer;
mod html_generator;
mod token;

use lexer::{Lexer};
use html_generator::{html_generator, file_generator};

// use crate::token::{Token, TokenType};

fn main() -> std::io::Result<()>{
    let python_code = String::from("#frutas = ['manzana', 'plátano', 'mango']
for fruta in frutas:
    print(fruit.capitalize())   hola

# Output: Manzana
#         Plátano
#         Mango");
    let mut lexer = Lexer::new(python_code);
    let tokens = lexer.tokenize();
    let html_content = html_generator(tokens);
    file_generator(html_content)?;
    println!("Archivo htmlgenerado correctamente");
    Ok(())
}
