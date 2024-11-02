pub mod evaluator;
pub mod parser;

use crate::{expression::Expression, token::Token};

pub enum Statement<'a> {
  Empty(),
  Print(Box<Expression<'a>>),
  Expression(Box<Expression<'a>>),
  Var(&'a Token, Box<Expression<'a>>),
  Block(Box<Vec<Statement<'a>>>),
  If(Box<Expression<'a>>, Box<Statement<'a>>, Box<Statement<'a>>),
  While(Box<Expression<'a>>, Box<Statement<'a>>)
}

impl<'a> Statement<'a> {
  pub fn to_string(&self) -> String {
    match self {
      Statement::Empty() => String::new(),
      Statement::Print(expression) => {
        format!("Print: {}", expression.to_string())
      },
      Statement::Expression(expression) => {
        format!("Expression: {}", expression.to_string())
      },
      Statement::Var(token, expression) => {
        format!("Var: {}, {}", token.lexeme, expression.to_string())
      },
      Statement::Block(statements) => {
        let str = statements.iter()
          .map(|s| format!("  {}", s.to_string()))
          .collect::<Vec<_>>()
          .join("\n  ");

        format!("Statements: \n  {}", str)
      },
      Statement::If(expr, then_stmt, else_stmt) => {
        format!("If:\n  Condition: {};\n  Then: {};\n  Else: {};", expr.to_string(), then_stmt.to_string(), else_stmt.to_string())
      },
      Statement::While(expr, stmt) => {
        format!("While:\n Condition: {};\n Statement: {}", expr.to_string(), stmt.to_string())
      }
    }
  }
}