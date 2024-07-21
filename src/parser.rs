use crate::token::{Token, TokenType};
use crate::expression::Expression;

pub struct Parser;

#[derive(Debug)]
pub enum ParseError {
  MissingToken(),
  UnmatchedParentheses(String),
}

impl ParseError {
  pub fn to_string(&self) -> String {
    match self {
      ParseError::MissingToken() => String::from(""),
      ParseError::UnmatchedParentheses(e) => e.to_string()
    }
  }
}

impl Parser {
  pub fn parse<'a>(tokens: &'a Vec<Token>) -> Result<Expression<'a>, ParseError> {
    Parser::expression(tokens, &mut 0)
  }

  fn expression<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParseError> {
    Parser::equality(tokens, index)
  }

  fn equality<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParseError> {
    let left = Parser::comparison(tokens, index);

    if left.is_err() {
      return left
    }

    let mut expression = left.unwrap();
    loop {
      match Parser::check_token(tokens, index, &[TokenType::BangEqual, TokenType::Equal]) {
        Some(token) => {
          let right = Parser::comparison(tokens, index);

          if right.is_err() {
            return right;
          }

          expression = Expression::Binary(Box::new(expression), token, Box::new(right.unwrap()));
        },
        None => break
      }
    }

    Ok(expression)
  }

  fn comparison<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParseError> {
    let left = Parser::term(tokens, index);

    if left.is_err() {
      return left
    }

    let mut expression = left.unwrap();
    loop {
      match Parser::check_token(tokens, index, &[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual, TokenType::EqualEqual]) {
        Some(token) => {
          let right = Parser::term(tokens, index);

          if right.is_err() {
            return right;
          }

          expression = Expression::Binary(Box::new(expression), token, Box::new(right.unwrap()));
        },
        None => break
      }
    }

    Ok(expression)
  }

  fn term<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParseError> {
    let left = Parser::factor(tokens, index);

    if left.is_err() {
      return left;
    }

    let mut expression = left.unwrap();
    loop {
      match Parser::check_token(tokens, index, &[TokenType::Minus, TokenType::Plus]) {
        Some(token) => {
          let right = Parser::factor(tokens, index);

          if right.is_err() {
            return right;
          }

          expression = Expression::Binary(Box::new(expression), token, Box::new(right.unwrap()));
        },
        None => break
      }
    }

    Ok(expression)
  }

  fn factor<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParseError> {
    let left = Parser::unary(tokens, index);

    if left.is_err() {
      return left;
    }

    let mut expression = left.unwrap();
    loop {
      match Parser::check_token(tokens, index, &[TokenType::Slash, TokenType::Star]) {
        Some(token) => {
          let right = Parser::unary(tokens, index);

          if right.is_err() {
            return right;
          }

          expression = Expression::Binary(Box::new(expression), token, Box::new(right.unwrap()));
        },
        None => break
      }
    }

    Ok(expression)
  }

  fn unary<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParseError> {
    match Parser::check_token(tokens, index, &[TokenType::Bang, TokenType::Minus]) {
      Some(token) => {
        let right = Parser::unary(tokens, index);

        if right.is_err() {
          return right;
        }

        Ok(Expression::Unary(token, Box::new(right.unwrap())))
      },
      None => Parser::primary(tokens, index)
    }
  }

  fn primary<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParseError> {
    match Parser::check_token(tokens, index, &[TokenType::False, TokenType::True, TokenType:: Nil]) {
      Some(token) => return Ok(Expression::Literal(format!("{}", token.lexeme))),
      None => {}
    };

    match Parser::check_token(tokens, index, &[TokenType::Number, TokenType::String]) {
      Some(token) => return Ok(Expression::Literal(format!("{}", token.literal))),
      None => {}
    };

    match Parser::check_token(tokens, index, &[TokenType::LeftParen]) {
      Some(_) => {
        let expression = Parser::expression(tokens, index);

        if expression.is_err() {
          return expression;
        }
        
        match Parser::check_token(tokens, index, &[TokenType::RightParen]) {
          Some(_) => {
            return Ok(Expression::Grouping(Box::new(expression.unwrap())));
          },
          None => Err(ParseError::UnmatchedParentheses(format!("Error: Unmatched parentheses.")))
        }
      },
      None => Err(ParseError::MissingToken())
    }  
  }

  /** Utils **/
  fn check_token<'a>(tokens: &'a Vec<Token>, index: &mut usize, token_types: &[TokenType]) -> Option<&'a Token> {
    if index >= &mut tokens.len() { return None; }
    
    let token = &tokens[index.to_owned()];

    if Parser::matches(token, &[TokenType::EOL, TokenType::EOF]) { return None; }
    if Parser::matches(token, token_types) {
      *index += 1;
      return Some(token);
    }

    None
  }

  fn matches(token: &Token, token_types: &[TokenType]) -> bool {
    for token_type in token_types {
      if token_type == &token.token_type {
        return true;
      }
    }

    false
  }
}
