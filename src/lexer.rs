use crate::token::{Token, TokenType};

// #[derive(Debug, Clone)]
pub struct Lexer{
    input: Vec<char>,
    current_pos: usize,
}

impl Lexer{
    pub fn new(input: String) -> Self{
        Self{
            input: input.chars().collect(),
            current_pos: 0,
        }
    }
    
    pub fn is_operator(&self, ch: char) -> bool{
        matches!(ch, '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | '&')
    }

    fn advance(&mut self){
        self.current_pos += 1;
    }
    
    fn current(&self) -> Option<char>{
        self.input.get(self.current_pos).copied()
    }
    
    fn peek(&self) -> Option<char>{
        self.input.get(self.current_pos+1).copied()
    }

    fn peek_non_whitespace(&self) -> Option<char> {
        let mut i = self.current_pos;

        while let Some(c) = self.input.get(i) {
            if c.is_whitespace() {
                i += 1;
            } else {
                return Some(*c);
            }
        }
        None
    }
    
    fn read_whitespace(&mut self) -> Token {
        let mut temp = String::new();
    
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                if c == ' '{
                    temp.push_str("&nbsp;");
    
                } else if c == '\n' {
                    temp.push_str("<br>");
    
                } else if c == '\t' {
                    temp.push_str("&emsp;");
    
                }
                self.advance();
    
            } else {
                break;
                
            }
        }
    
        Token::new(temp, TokenType::Whitespace)
    }
    
    fn read_comment(&mut self) -> Token{
        let mut temp = String::new();
    
        while let Some(c) = self.current() {
            if c == '\n' {
                break;
            } else {
                temp.push(c);
                self.advance();
            }
        }
        Token::new(temp, TokenType::Comments)
    }
 
    fn read_string(&mut self) -> Token {
        let mut temp = String::new();

        // Guardar el delimitador (' o ")
        let quote = self.current().unwrap();
        temp.push(quote);
        self.advance();

        while let Some(c) = self.current() {
            temp.push(c);
            self.advance();

            if c == quote {
                break;
            }
        }

        Token::new(temp, TokenType::String)
    }
    
    fn read_operator(&mut self) -> Token {
        let mut temp = String::new();

        if let Some(c) = self.current() {
            temp.push(c);

            // Verificar operadores compuestos
            if let Some(next) = self.peek() {
                let is_double = matches!(
                    (c, next),
                    ('=', '=') |
                    ('!', '=') |
                    ('<', '=') |
                    ('>', '=') |
                    ('&', '&') |
                    ('|', '|')
                );

                if is_double {
                    temp.push(next);
                    self.advance();
                }
            }

            self.advance();
        }

        Token::new(temp, TokenType::Operator)
    }
    
    fn read_number(&mut self) -> Token {
        let mut temp = String::new();
        let mut has_dot = false;

        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                temp.push(c);
                self.advance();
            } else if c == '.' && !has_dot {
                has_dot = true;
                temp.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token::new(temp, TokenType::Number)
    }
    
    fn read_word(&mut self) -> Token{
        let mut temp = String::new();
        let r_words = ["if", "else", "for", "while", "def", "return", "class", "import", "print"];
        let o_words = ["and", "or", "not", "in", "is"];
    
        while let Some(c) = self.current(){
            if c.is_alphabetic() || c == '_' || c.is_ascii_digit(){
                temp.push(c);
                self.advance();
            } else {
                break;
            }
        }
    
        if r_words.contains(&temp.as_str()){
            Token::new(temp, TokenType::Keyword)
        } else if o_words.contains(&temp.as_str()){
            Token::new(temp, TokenType::Operator)
        }  else if self.peek_non_whitespace() == Some('(') {
            Token::new(temp, TokenType::None)
        } else {
            Token::new(temp, TokenType::Variable)
        }
    }
    

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current(){
            if ch.is_whitespace(){
                tokens.push(self.read_whitespace());
            } else if ch == '#' {
                tokens.push(self.read_comment());
            } else if ch == '"' || ch == '\''{
                tokens.push(self.read_string());
            } else if self.is_operator(ch){
                tokens.push(self.read_operator());
            } else if ch.is_ascii_digit(){
                tokens.push(self.read_number());
            } else if ch.is_alphabetic() || ch == '_' { //|| ch == '.'{
                tokens.push(self.read_word())
            } else {
                tokens.push(Token::new(ch.to_string(), TokenType::None));
                self.advance();
            }
        }

        tokens
    }

}