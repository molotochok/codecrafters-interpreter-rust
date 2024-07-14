pub struct Token {
  pub token_type: TokenTypes,
  pub lexeme: String,
  pub literal: String
}

impl Token {
  pub fn to_str(&self) -> String {
      format!("{} {} {}", self.token_type.to_str(), self.lexeme, self.literal)
  }
}



pub enum TokenTypes {
  // *** Single Character ***
  // (
  LeftParen,
  // )
  RightParen,
  // {
  LeftBrace,
  // }
  RightBrace,
  // ,
  Comma,
  // .
  Dot,
  // +
  Plus,
  // *
  Star,
  // -
  Minus,
  // ;
  Semicolon,
  // /
  Slash, 

  EOF
}

impl TokenTypes {
  pub fn from_char(c: char) -> TokenTypes {
      match c {
          '(' => TokenTypes::LeftParen,
          ')' => TokenTypes::RightParen,
          '{' => TokenTypes::LeftBrace,
          '}' => TokenTypes::RightBrace,
          ',' => TokenTypes::Comma,
          '.' => TokenTypes::Dot,
          '+' => TokenTypes::Plus,
          '*' => TokenTypes::Star,
          '-' => TokenTypes::Minus,
          ';' => TokenTypes::Semicolon,
          '/' => TokenTypes::Slash,
          _ => panic!("An invalid token type")
      }
  }

  pub fn to_str(&self) -> &'static str {
      match self {
          // *** Single Character ***
          TokenTypes::LeftParen => "LEFT_PAREN",
          TokenTypes::RightParen => "RIGHT_PAREN",
          TokenTypes::LeftBrace => "LEFT_BRACE",
          TokenTypes::RightBrace => "RIGHT_BRACE",
          TokenTypes::Comma => "COMMA",
          TokenTypes::Dot => "DOT",
          TokenTypes::Plus => "PLUS",
          TokenTypes::Star => "STAR",
          TokenTypes::Minus => "MINUS",
          TokenTypes::Semicolon => "SEMICOLON",
          TokenTypes::Slash => "SLASH",
          TokenTypes::EOF => "EOF"
      }
  }
}