use std::borrow::Cow;
use crate::token::Token;

pub enum Expression<'a> {
  Literal(String),
  Unary(&'a Token, Box<Expression<'a>>),
  Binary(Box<Expression<'a>>, &'a Token, Box<Expression<'a>>),
  Grouping(Box<Expression<'a>>),
}

impl<'a> Expression<'a> {
  pub fn to_string(&self) -> String {
    match self {
      Expression::Literal(literal) => literal.to_owned(),
      Expression::Unary(token, right) => Expression::parenthesize(&token.lexeme, &[right]),
      Expression::Binary(left, token, right) => Expression::parenthesize(&token.lexeme, &[left, right]),
      Expression::Grouping(expr) => Expression::parenthesize(&Cow::Borrowed("group"), &[expr]),
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