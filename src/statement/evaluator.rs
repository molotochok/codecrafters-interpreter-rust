use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, expression::{evaluator::ExprEvaluator, expr_eval_error::ExprEvalError}};
use super::Statement;

pub enum StmtEvalError {
  ExpressionError(ExprEvalError)
}
impl StmtEvalError {
  pub fn to_string(&self) -> String {
    match self {
      StmtEvalError::ExpressionError(e) => format!("Statement failure: {}", e.to_string()),
    }
  }
}

pub struct StmtEvaluator;

impl StmtEvaluator {
  pub fn evaluate<'a>(statement: &'a Statement, env: &Rc<RefCell<Environment>>) -> Result<(), StmtEvalError> {
    match statement {
      Statement::Print(e) => {
        match ExprEvaluator::evaluate(e, env) {
          Ok(t) => {
            println!("{}", t.to_string());
            Ok(())
          },
          Err(e) => Err(StmtEvalError::ExpressionError(e))
        }
      },
      Statement::Expression(e) => {
        match ExprEvaluator::evaluate(e, env) {
          Ok(_r) => Ok(()),
          Err(e) => Err(StmtEvalError::ExpressionError(e))
        }
      },
      Statement::Var(token, e) => {
        match ExprEvaluator::evaluate(e, env) {
          Ok(t) => {
            env.borrow_mut().define(token.lexeme.to_string(), t);
            return Ok(());
          },
          Err(e) => Err(StmtEvalError::ExpressionError(e))
        }
      },
      Statement::Block(statements) => {
        let local_env = Rc::new(RefCell::new(Environment::local(env.clone())));
        
        for statement in statements.as_ref() {
          let res = StmtEvaluator::evaluate(&statement, &Rc::clone(&local_env));
          
          if res.is_err() {
            return res;
          }
        }

        Ok(())
      }
    }
  }
}