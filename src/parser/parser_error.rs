use crate::token::TokenType;

#[derive(Debug)]
pub enum ParserError {
  MissingToken(TokenType),
  UnmatchedParentheses(),
  ExpectExpression(),
  InvalidAssignment(String)
}

impl ParserError {
  pub fn to_string(&self) -> String {
    match self {
      ParserError::MissingToken(t) => format!("Missing Token: {}.", t.to_string()),
      ParserError::UnmatchedParentheses() => format!("Error: Unmatched parentheses"),
      ParserError::ExpectExpression() => format!("Expect expression"),
      ParserError::InvalidAssignment(expr) => format!("{}", expr)
    }
  }
}