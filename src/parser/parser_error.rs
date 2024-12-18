use crate::token::TokenType;

#[derive(Debug)]
pub enum ParserError {
  MissingToken(TokenType),
  UnmatchedParentheses(),
  ExpectExpression(String),
  InvalidAssignment(String),
}

impl ParserError {
  pub fn to_string(&self) -> String {
    match self {
      ParserError::MissingToken(t) => format!("Missing Token: {}.", t.to_string()),
      ParserError::UnmatchedParentheses() => format!("Error: Unmatched parentheses"),
      ParserError::ExpectExpression(msg) => format!("Expect expression. {}", msg),
      ParserError::InvalidAssignment(expr) => format!("{}", expr)
    }
  }
}