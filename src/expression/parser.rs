use crate::parser::parser_error::ParserError;
use crate::parser::parser_utils::ParserUtils;
use crate::token::{Token, TokenType};
use crate::expression::Expression;

pub struct ExprParser;


impl ExprParser {
  pub fn parse<'a>(tokens: &'a Vec<Token>) -> Result<Expression<'a>, ParserError> {
    ExprParser::expression(tokens, &mut 0)
  }

  pub fn expression<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParserError> {
    ExprParser::assignment(tokens, index)
  }

  pub fn assignment<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParserError> {
    match ExprParser::equality(tokens, index) {
      Ok(expr) => {
        match ParserUtils::match_advance(tokens, index, &[TokenType::Equal]) {
          Some(_equal) => {
            match ExprParser::assignment(tokens, index) {
              Ok(value) => match expr {
                Expression::Variable(token) => return Ok(Expression::Assign(token, Box::new(value))),
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

  fn equality<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParserError> {
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

  fn comparison<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParserError> {
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

  fn term<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParserError> {
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

  fn factor<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParserError> {
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

  fn unary<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParserError> {
    match ParserUtils::match_advance(tokens, index, &[TokenType::Bang, TokenType::Minus]) {
      Some(token) => {
        let right = ExprParser::unary(tokens, index);

        if right.is_err() {
          return right;
        }

        Ok(Expression::Unary(token, Box::new(right.unwrap())))
      },
      None => ExprParser::primary(tokens, index)
    }
  }

  fn primary<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParserError> {
    match ParserUtils::match_advance(tokens, index, &[TokenType::False, TokenType::True, TokenType:: Nil, TokenType::Number, TokenType::String]) {
      Some(token) => return Ok(Expression::Literal(token)),
      None => {}
    };

    match ParserUtils::match_advance(tokens, index, &[TokenType::Identifier]) {
      Some(token) => return Ok(Expression::Variable(token)),
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
      None => Err(ParserError::ExpectExpression())
    }  
  }
}
