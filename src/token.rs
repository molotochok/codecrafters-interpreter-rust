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

#[derive(PartialEq)]
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

  // /n
  EOL,
  EOF
}

impl TokenTypes {
  pub fn from_char(c: char) -> Result<TokenTypes, String> {
      match c {
          '(' => Ok(TokenTypes::LeftParen),
          ')' => Ok(TokenTypes::RightParen),
          '{' => Ok(TokenTypes::LeftBrace),
          '}' => Ok(TokenTypes::RightBrace),
          ',' => Ok(TokenTypes::Comma),
          '.' => Ok(TokenTypes::Dot),
          '+' => Ok(TokenTypes::Plus),
          '*' => Ok(TokenTypes::Star),
          '-' => Ok(TokenTypes::Minus),
          ';' => Ok(TokenTypes::Semicolon),
          '/' => Ok(TokenTypes::Slash),
          '\n' => Ok(TokenTypes::EOL),
          _ => Err(String::from("An invalid token type"))
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
          TokenTypes::EOL => "EOL",
          TokenTypes::EOF => "EOF"
      }
  }
}