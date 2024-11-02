pub mod parser;
pub mod evaluator;
pub mod expr_type;
pub mod expr_eval_error;

use std::borrow::Cow;
use crate::token::{Token, TokenType};

pub enum Expression<'a> {
  Literal(&'a Token),
  Unary(&'a Token, Box<Expression<'a>>),
  Binary(Box<Expression<'a>>, &'a Token, Box<Expression<'a>>),
  Grouping(Box<Expression<'a>>),
  Variable(&'a Token),
  Assign(&'a Token, Box<Expression<'a>>),
  Logical(Box<Expression<'a>>, &'a Token, Box<Expression<'a>>),
  Nil(),
}

impl<'a> Expression<'a> {
  pub fn to_string(&self) -> String {
    match self {
      Expression::Literal(token) => {
        match token.token_type {
          TokenType::Nil | TokenType::False | TokenType::True => token.lexeme.to_string(),
          _ => token.literal.to_string()
        }
      },
      Expression::Unary(token, right) => Expression::parenthesize(&token.lexeme, &[right]),
      Expression::Binary(left, token, right) => Expression::parenthesize(&token.lexeme, &[left, right]),
      Expression::Grouping(expr) => Expression::parenthesize(&Cow::Borrowed("group"), &[expr]),
      Expression::Variable(token) => token.lexeme.to_string(),
      Expression::Assign(token, expression) => format!("{} = {}", token.lexeme, expression.to_string()),
      Expression::Logical(left, operator, right) => format!("{} {} {}", left.to_string(), operator.to_str(), right.to_string()),
      Expression::Nil() => format!("nil")
    }
  }

  fn parenthesize(name: &Cow<'static, str>, expressions: &[&Box<Expression>]) -> String {
    let mut expr_str = format!("({}", name);

    for expression in expressions {
      expr_str.push_str(&format!(" {}", expression.to_string()));
    }

    format!("{})", expr_str)
  }
}