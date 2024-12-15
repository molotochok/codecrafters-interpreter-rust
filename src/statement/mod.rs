pub mod evaluator;
pub mod parser;


use std::rc::Rc;

use crate::{expression::Expression, runtime::{runtime_error::RuntimeError, runtime_type::RuntimeType}, token::Token};

#[derive(Clone)]
pub enum Statement {
  Empty(),
  Print(Box<Expression>),
  Expression(Box<Expression>),
  Var(Rc<Token>, Box<Expression>),
  Function(Rc<Token>, Vec<Rc<Token>>, Box<Statement>),
  Block(Box<Vec<Statement>>),
  If(Box<Expression>, Box<Statement>, Box<Statement>),
  While(Box<Expression>, Box<Statement>),
  
  // Used for native functions.
  Native(Rc<dyn Fn() -> Result<RuntimeType, RuntimeError>>)
}

impl Statement {
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
      Statement::Function(func_name, args, body) => {
        format!("Function: name = {}, args = {}, body = {}", func_name.lexeme, args.iter().map(|a| a.to_str()).collect::<Vec<_>>().join(", ").to_string(), body.to_string())
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
      },
      Statement::Native(_fun) => {
        format!("Native function")
      },
    }
  }
}