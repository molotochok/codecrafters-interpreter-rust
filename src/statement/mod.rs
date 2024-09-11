pub mod evaluator;
pub mod parser;

use crate::{expression::Expression, token::Token};

pub enum Statement<'a> {
  Print(Box<Expression<'a>>),
  Expression(Box<Expression<'a>>),
  Var(&'a Token, Box<Expression<'a>>)
}

impl<'a> Statement<'a> {
  pub fn to_string(&self) -> String {
    match self {
      Statement::Print(expression) => {
        format!("Print: {}", expression.to_string())
      },
      Statement::Expression(expression) => {
        format!("Expression: {}", expression.to_string())
      },
      Statement::Var(token, expression) => {
        format!("Var: {}, {}", token.lexeme, expression.to_string())
      }
    }
  }
}