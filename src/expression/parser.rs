use std::rc::Rc;

use crate::parser::parser_error::ParserError;
use crate::parser::parser_utils::ParserUtils;
use crate::token::{Token, TokenType};
use crate::expression::Expression;

pub struct ExprParser;

impl ExprParser {
  pub fn parse(tokens: &Vec<Rc<Token>>) -> Result<Expression, ParserError> {
    ExprParser::expression(tokens, &mut 0)
  }

  pub fn expression(tokens: &Vec<Rc<Token>>, index: &mut usize) -> Result<Expression, ParserError> {
    ExprParser::assignment(tokens, index)
  }

  pub fn assignment(tokens: &Vec<Rc<Token>>, index: &mut usize) -> Result<Expression, ParserError> {
    match ExprParser::or(tokens, index) {
      Ok(expr) => {
        match ParserUtils::match_advance(tokens, index, &[TokenType::Equal]) {
          Some(_equal) => {
            match ExprParser::assignment(tokens, index) {
              Ok(value) => match expr {
                Expression::Identifier(token) => return Ok(Expression::Assign(token, Box::new(value))),
                _ => Err(ParserError::InvalidAssignment(expr.to_string())),
              },
              Err(_e) => Err(ParserError::InvalidAssignment(expr.to_string()))
            }
          },
          None => Ok(expr)
        }
      },
      Err(e) => Err(e)
    }
  }

  fn or(tokens: &Vec<Rc<Token>>, index: &mut usize) -> Result<Expression, ParserError> {
    let left = ExprParser::and(tokens, index);
    
    if left.is_err() {
      return left;
    }

    let mut expression = left.unwrap();
    loop {
      match ParserUtils::match_advance(tokens, index, &[TokenType::Or]) {
        Some(and) => match ExprParser::and(tokens, index) {
          Ok(right) => {
            expression = Expression::Logical(Box::new(expression), and, Box::new(right));
          },
          Err(e) => return Err(e)
        },
        None => break
      }
    }

    Ok(expression)
  }

  fn and(tokens: &Vec<Rc<Token>>, index: &mut usize) -> Result<Expression, ParserError> {
    let left = ExprParser::equality(tokens, index);
    
    if left.is_err() {
      return left;
    }

    let mut expression = left.unwrap();
    loop {
      match ParserUtils::match_advance(tokens, index, &[TokenType::And]) {
        Some(and) => match ExprParser::equality(tokens, index) {
          Ok(right) => {
            expression = Expression::Logical(Box::new(expression), and, Box::new(right));
          },
          Err(e) => return Err(e)
        },
        None => break
      }
    }

    Ok(expression)
  }

  fn equality(tokens: &Vec<Rc<Token>>, index: &mut usize) -> Result<Expression, ParserError> {
    let left = ExprParser::comparison(tokens, index);

    if left.is_err() {
      return left
    }

    let mut expression = left.unwrap();
    loop {
      match ParserUtils::match_advance(tokens, index, &[TokenType::BangEqual, TokenType::EqualEqual]) {
        Some(token) => {
          let right = ExprParser::comparison(tokens, index);

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

  fn comparison(tokens: &Vec<Rc<Token>>, index: &mut usize) -> Result<Expression, ParserError> {
    let left = ExprParser::term(tokens, index);

    if left.is_err() {
      return left
    }

    let mut expression = left.unwrap();
    loop {
      match ParserUtils::match_advance(tokens, index, &[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual, TokenType::EqualEqual]) {
        Some(token) => {
          let right = ExprParser::term(tokens, index);

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

  fn term(tokens: &Vec<Rc<Token>>, index: &mut usize) -> Result<Expression, ParserError> {
    let left = ExprParser::factor(tokens, index);

    if left.is_err() {
      return left;
    }

    let mut expression = left.unwrap();
    loop {
      match ParserUtils::match_advance(tokens, index, &[TokenType::Minus, TokenType::Plus]) {
        Some(token) => {
          let right = ExprParser::factor(tokens, index);

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

  fn factor(tokens: &Vec<Rc<Token>>, index: &mut usize) -> Result<Expression, ParserError> {
    let left = ExprParser::unary(tokens, index);

    if left.is_err() {
      return left;
    }

    let mut expression = left.unwrap();
    loop {
      match ParserUtils::match_advance(tokens, index, &[TokenType::Slash, TokenType::Star]) {
        Some(token) => {
          let right = ExprParser::unary(tokens, index);

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

  fn unary(tokens: &Vec<Rc<Token>>, index: &mut usize) -> Result<Expression, ParserError> {
    match ParserUtils::match_advance(tokens, index, &[TokenType::Bang, TokenType::Minus]) {
      Some(token) => {
        let right = ExprParser::unary(tokens, index);

        if right.is_err() {
          return right;
        }

        Ok(Expression::Unary(token, Box::new(right.unwrap())))
      },
      None => ExprParser::call(tokens, index)
    }
  }

  fn call(tokens: &Vec<Rc<Token>>, index: &mut usize) -> Result<Expression, ParserError> {
    match ExprParser::primary(tokens, index) {
      Ok(callee) => {
        // TODO: This should be in a loop and I should cover closing paren
        match ParserUtils::match_advance(tokens, index, &[TokenType::LeftParen]) {
          Some(_lp) => {
            match ExprParser::arguments(tokens, index) {
              Ok(arguments) => {
                match ParserUtils::match_advance(tokens, index, &[TokenType::RightParen]) {
                  Some(_rp) => Ok(Expression::Call(Box::new(callee), arguments)),
                  None => Err(ParserError::MissingToken(TokenType::RightParen))
                }
              },
              Err(e) => Err(e)
            }
          },
          None => Ok(callee)
        }
      },
      Err(e) => Err(e)
    }
  }

  fn arguments(tokens: &Vec<Rc<Token>>, index: &mut usize) -> Result<Vec<Expression>, ParserError> {
    let mut arguments: Vec<Expression> = Vec::new();

    if ParserUtils::matches(&tokens[*index], &[TokenType::RightParen]) {
      return Ok(arguments);
    }

    match ExprParser::expression(tokens, index) {
      Ok(arg) => arguments.push(arg),
      Err(e) => return Err(e)
    };

    loop {
      match ParserUtils::match_advance(tokens, index, &[TokenType::Comma]) {
        Some(_c) => {
          match ExprParser::expression(tokens, index) {
            Ok(arg) => arguments.push(arg),
            Err(e) => return Err(e)
          }
        },
        None => break
      }
    }

    Ok(arguments)
  }

  fn primary(tokens: &Vec<Rc<Token>>, index: &mut usize) -> Result<Expression, ParserError> {
    match ParserUtils::match_advance(tokens, index, &[TokenType::False, TokenType::True, TokenType:: Nil, TokenType::Number, TokenType::String]) {
      Some(token) => return Ok(Expression::Literal(token)),
      None => {}
    };

    match ParserUtils::match_advance(tokens, index, &[TokenType::Identifier]) {
      Some(token) => return Ok(Expression::Identifier(token)),
      None => {}
    };

    match ParserUtils::match_advance(tokens, index, &[TokenType::LeftParen]) {
      Some(_) => {
        let expression = ExprParser::expression(tokens, index);

        if expression.is_err() {
          return expression;
        }
        
        match ParserUtils::match_advance(tokens, index, &[TokenType::RightParen]) {
          Some(_) => {
            return Ok(Expression::Grouping(Box::new(expression.unwrap())));
          },
          None => Err(ParserError::UnmatchedParentheses())
        }
      },
      None => Err(ParserError::ExpectExpression(String::from("Expected LeftParen, got nothing.")))
    }  
  }
}
