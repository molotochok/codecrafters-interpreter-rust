use crate::token::{Token, TokenType};
use crate::expression::Expression;

pub struct Parser;

impl Parser {
  pub fn parse<'a>(tokens: &'a Vec<Token>) -> Expression<'a> {
    Parser::expression(tokens, &mut 0)
  }

  fn expression<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Expression<'a> {
    Parser::equality(tokens, index)
  }

  fn equality<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Expression<'a> {
    let mut expression = Parser::comparison(tokens, index);

    loop {
      match Parser::check_token(tokens, index, &[TokenType::BangEqual, TokenType::Equal]) {
        Some(token) => {
          let right = Parser::comparison(tokens, index);
          expression = Expression::Binary(Box::new(expression), token, Box::new(right))
        },
        None => break
      }
    }

    expression
  }

  fn comparison<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Expression<'a> {
    let mut expression = Parser::term(tokens, index);

    loop {
      match Parser::check_token(tokens, index, &[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
        Some(token) => {
          let right = Parser::term(tokens, index);
          expression = Expression::Binary(Box::new(expression), token, Box::new(right))
        },
        None => break
      }
    }

    expression
  }

  fn term<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Expression<'a> {
    let mut expression = Parser::factor(tokens, index);

    loop {
      match Parser::check_token(tokens, index, &[TokenType::Minus, TokenType::Plus]) {
        Some(token) => {
          let right = Parser::factor(tokens, index);
          expression = Expression::Binary(Box::new(expression), token, Box::new(right))
        },
        None => break
      }
    }

    expression
  }

  fn factor<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Expression<'a> {
    let mut expression = Parser::unary(tokens, index);

    loop {
      match Parser::check_token(tokens, index, &[TokenType::Slash, TokenType::Star]) {
        Some(token) => {
          let right = Parser::unary(tokens, index);
          expression = Expression::Binary(Box::new(expression), token, Box::new(right))
        },
        None => break
      }
    }

    expression
  }

  fn unary<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Expression<'a> {
    match Parser::check_token(tokens, index, &[TokenType::Bang, TokenType::Minus]) {
      Some(token) => {
        let right = Parser::unary(tokens, index);
        Expression::Unary(token, Box::new(right))
      },
      None => Parser::primary(tokens, index)
    }
  }

  fn primary<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Expression<'a> {
    match Parser::check_token(tokens, index, &[TokenType::False, TokenType::True, TokenType:: Nil]) {
      Some(token) => return Expression::Literal(format!("{}", token.lexeme)),
      None => {}
    };

    match Parser::check_token(tokens, index, &[TokenType::Number, TokenType::String]) {
      Some(token) => return Expression::Literal(format!("{}", token.literal)),
      None => {}
    };

    match Parser::check_token(tokens, index, &[TokenType::LeftParen]) {
      Some(_) => {
        let expression = Parser::expression(tokens, index);
        
        match Parser::check_token(tokens, index, &[TokenType::RightParen]) {
          Some(_) => {
            return Expression::Grouping(Box::new(expression));
          },
          None => Expression::Literal(format!("{}", Token::EOF.literal))
        }
      },
      None => Expression::Literal(format!("{}", Token::EOF.literal))
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
