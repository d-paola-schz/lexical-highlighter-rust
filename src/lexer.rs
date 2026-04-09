use crate::token::{Token, TokenType};

// #[derive(Debug, Clone)]
pub struct Lexer{
    input: vec<char>,
    currentPos: usize,
}

impl Lexer{
    pub fn new(input: String) -> Self{
        Self{
            input: input.chars().collect(),
            currentPos: 0,
        }
    }
}