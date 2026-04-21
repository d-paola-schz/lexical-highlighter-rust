#[derive(Debug, Clone, PartialEq)]
pub enum TokenType{
  Keyword,
  Variable,
  String,
  Number,
  Operator,
  Comments,
  Whitespace,
  None,
}

impl TokenType {
    pub fn to_str(&self) -> &str {
        match self {
            TokenType::Keyword => "Keyword",
            TokenType::Variable => "Variable",
            TokenType::String => "String",
            TokenType::Number => "Number",
            TokenType::Operator => "Operator",
            TokenType::Comments => "Comments",
            TokenType::Whitespace => "Whitespace",
            TokenType::None => "None",
        }
    }
}


#[derive(Debug, Clone)]
pub struct Token{
  pub text: String,
  pub token_type: TokenType,
}

impl Token {
  pub fn new(text: String, token_type: TokenType) -> Self {
    Self {text, token_type}
  }
}
