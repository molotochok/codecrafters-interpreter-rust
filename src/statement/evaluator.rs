use crate::{environment::Environment, expression::{evaluator::ExprEvaluator, expr_eval_error::ExprEvalError, expr_type::ExprType}};
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
  pub fn evaluate(statement: &Statement, env: &mut Environment) -> Result<Option<ExprType>, StmtEvalError> {
    match statement {
      Statement::Print(e) => {
        match ExprEvaluator::evaluate(e, env) {
          Ok(t) => Ok(Some(t)),
          Err(e) => Err(StmtEvalError::ExpressionError(e))
        }
      },
      Statement::Expression(e) => {
        match ExprEvaluator::evaluate(e, env) {
          Ok(_r) => return Ok(None),
          Err(e) => Err(StmtEvalError::ExpressionError(e))
        }
      },
      Statement::Var(token, e) => {
        match ExprEvaluator::evaluate(e, env) {
          Ok(t) => {
            env.define(token.lexeme.to_string(), t);
            return Ok(None)
          },
          Err(e) => Err(StmtEvalError::ExpressionError(e))
        }
      }
    }
  }
}