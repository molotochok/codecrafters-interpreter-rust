use crate::token::{Token, TokenType};

pub struct ParserUtils;

impl ParserUtils {
  pub fn match_advance<'a>(tokens: &'a Vec<Token>, index: &mut usize, token_types: &[TokenType]) -> Option<&'a Token> {
    if index >= &mut tokens.len() { return None; }
    
    let token = &tokens[index.to_owned()];

    if ParserUtils::matches(token, &[TokenType::EOL, TokenType::EOF, TokenType::Semicolon]) { return None; }
    if ParserUtils::matches(token, token_types) {
      *index += 1;
      return Some(token);
    }

    None
  }

  pub fn matches(token: &Token, token_types: &[TokenType]) -> bool {
    for token_type in token_types {
      if token_type == &token.token_type {
        return true;
      }
    }

    false
  }
}