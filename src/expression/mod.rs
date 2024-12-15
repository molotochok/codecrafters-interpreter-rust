pub mod parser;
pub mod evaluator;

use std::{borrow::Cow, rc::Rc};
use crate::token::{Token, TokenType};

#[derive(Clone)]
pub enum Expression {
  Literal(Rc<Token>),
  Unary(Rc<Token>, Box<Expression>),
  Binary(Box<Expression>, Rc<Token>, Box<Expression>),
  Grouping(Box<Expression>),
  Identifier(Rc<Token>),
  Assign(Rc<Token>, Box<Expression>),
  Logical(Box<Expression>, Rc<Token>, Box<Expression>),
  Call(Box<Expression>, Vec<Expression>),
  Nil(),
}

impl Expression {
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
      Expression::Identifier(token) => token.lexeme.to_string(),
      Expression::Assign(token, expression) => format!("{} = {}", token.lexeme, expression.to_string()),
      Expression::Logical(left, operator, right) => format!("{} {} {}", left.to_string(), operator.to_str(), right.to_string()),
      Expression::Call(calle, arguments) => format!("{}{}", calle.to_string(), arguments.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")),
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