#[derive(Debug, Clone, PartialEq)]
pub enum TokenType{
  Keyword,
  Variable,
  String,
  Number,
  Operator,
  Comments,
  Whitespace,
}

#[derive(Debug, Clone)]
struct Token{
  pub text: String,
  pub tokenType: TokenType,
}

impl Token {
  pub fn new(text: String, tokenType: TokenType) -> Self {
    Self {text, tokenType}
  }
}
