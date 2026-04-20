use crate::token::{Token, TokenType};

// #[derive(Debug, Clone)]
pub struct Lexer{
    input: Vec<char>,
    currentPos: usize,
}

impl Lexer{
    pub fn new(input: String) -> Self{
        Self{
            input: input.chars().collect(),
            currentPos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current(){
            if ch.is_whitespace(){
                tokens.push(self.readWhitespace());
            } else if ch == '#' {
                tokens.push(self.readComment());
            } else if ch == '"' || ch == '\''{
                tokens.push(self.readString());
            } else if is_operator(ch){
                tokens.push(self.readOperator());
            } else if ch.is_ascii_digit(){
                tokens.push(self.readNumber());
            } else if ch.is_alphabetic() || ch == '_'{
                tokens.push(self.readWord())
            } else {
                self.advance();
            }
        }

        tokens
    }

    fn is_operator(&self, ch: char) -> bool{
        matches!(ch, '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | '&')
    }

    fn advance(&mut self){
        self.currentPos += 1;
    }

    fn current(&self) -> Option<char>{
        self.input.get(self.currentPos).copied()
    }

    fn peek(&self) -> Option<char>{
        self.input.get(self.currentPos+1).copied()
    }

    fn readWhitespace(&mut self) -> Token {
        let mut temp = String::new();

        while let Some(c) = self.current() {
            if c.is_whitespace() {
                temp.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token::new(temp, TokenType::Whitespace)
    }

    fn readComment(&mut self) -> Token{
        let mut temp = String::new();

        while let Some(c) = self.current() {
            if c == '\n' {
                break;
            } else {
                temp.push(c);
                self.advance();
            }
        }
    }

    fn readString(&mut self) -> Token{
        unimplemented!() 
    }

    fn readOperator(&mut self) -> Token{
        unimplemented!() 
    }

    fn readNumber(&mut self) -> Token{
        unimplemented!() 
    }

    fn readWord(&mut self) -> Token{
        let mut temp = String::new();
        let rWords = ["if", "else", "for", "while", "def", "return", "class", "import"];
        let oWords = ["and", "or", "not", "in", "is"];

        while let Some(c) = self.current(){
            if c.is_alphabetic() || c == '_' || c.is_ascii_digit(){
                temp.push(c);
                self.advance();
            }  else {
                break;
            }
        }

        if rWords.contains(&temp.as_str()){
            Token::new(temp, TokenType::Keyword)
        } else if oWords.contains(&temp.as_str()){
            Token::new(temp, TokenType::Operator)
        } else {
            Token::new(temp, TokenType::Variable)
        }
    }

}