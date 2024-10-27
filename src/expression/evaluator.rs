use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::{expression::Expression, token::TokenType};
use crate::expression::expr_type::ExprType;
use crate::expression::expr_eval_error::ExprEvalError;

pub struct ExprEvaluator;

impl ExprEvaluator {
  pub fn evaluate<'a>(expression: &'a Expression, env: &Rc<RefCell<Environment>>) -> Result<ExprType, ExprEvalError> {
    match expression {
      Expression::Nil() => Ok(ExprType::Nil()),
      Expression::Assign(token, expression) => {
        match ExprEvaluator::evaluate(expression, env) {
          Ok(value) => {
            match env.borrow_mut().assign(token.lexeme.to_string(), value.clone()) {
              Ok(()) => Ok(value),
              Err(e) => Err(e)
            }
          },
          Err(e) => Err(e)
        }
      },
      Expression::Variable(token) => {
        match env.borrow().get(&token.lexeme.to_string()) {
          Some(v) => Ok(v.into_inner()),
          None => Err(ExprEvalError::UndefinedVariable(token.lexeme.to_string()))
        }
      },
      Expression::Literal(token) => {
        match token.token_type {
          TokenType::Nil => Ok(ExprType::Nil()),
          TokenType::True | TokenType::False => Ok(ExprType::Boolean(token.lexeme.parse::<bool>().unwrap())),
          TokenType::Number => Ok(ExprType::Number(token.literal.parse::<f64>().unwrap())),
          TokenType::String => Ok(ExprType::String(token.literal.to_string())),
          _ => Ok(ExprType::Nil())
        }
      },
      Expression::Grouping(e) => ExprEvaluator::evaluate(e, env),
      Expression::Unary(token, e) => {
        let value = ExprEvaluator::evaluate(e, env);

        match value {
          Ok(v) => {
            match token.token_type {
              TokenType::Bang => Ok(ExprType::Boolean(!v.is_truthy())),
              TokenType::Minus => match v {
                ExprType::Number(n) => Ok(ExprType::Number(-n)),
                _ => Err(ExprEvalError::UnaryError(format!("Operand must be a number.\n[line {}]", token.line)))
              },
              _ => Ok(ExprType::Nil())
            }
          },
          Err(e) => Err(e)
        }
      },
      Expression::Binary(left, token, right) => {
        let left_value_r = ExprEvaluator::evaluate(left, env);
      
        match left_value_r {
          Ok(left_value) => {
            let right_value_r = ExprEvaluator::evaluate(right, env);

            match right_value_r {
              Ok(right_value) => {
                match left_value {
                  ExprType::Number(ln) => match right_value {
                    ExprType::Number(rn) => match token.token_type {
                      TokenType::Plus => Ok(ExprType::Number(ln + rn)),
                      TokenType::Minus => Ok(ExprType::Number(ln - rn)),
                      TokenType::Star => Ok(ExprType::Number(ln * rn)),
                      TokenType::Slash => Ok(ExprType::Number(ln / rn)),
                      TokenType::Greater => Ok(ExprType::Boolean(ln > rn)),
                      TokenType::GreaterEqual => Ok(ExprType::Boolean(ln >= rn)),
                      TokenType::Less => Ok(ExprType::Boolean(ln < rn)),
                      TokenType::LessEqual => Ok(ExprType::Boolean(ln <= rn)),
                      TokenType::EqualEqual => Ok(ExprType::Boolean(ln == rn)),
                      TokenType::BangEqual => Ok(ExprType::Boolean(ln != rn)),
                      _ => Ok(ExprType::Nil())
                    },
                    ExprType::String(_rs) => match token.token_type {
                      TokenType::EqualEqual => Ok(ExprType::Boolean(false)),
                      TokenType::BangEqual => Ok(ExprType::Boolean(true)),
                      _ => Err(ExprEvalError::BinaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                    },
                    _ => Err(ExprEvalError::BinaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                  },
                  ExprType::String(ls) => match right_value {
                    ExprType::String(rs) => match token.token_type {
                      TokenType::Plus => Ok(ExprType::String(ls + &rs)),
                      TokenType::EqualEqual => Ok(ExprType::Boolean(ls == rs)),
                      TokenType::BangEqual => Ok(ExprType::Boolean(ls != rs)),
                      _ => Err(ExprEvalError::BinaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                    },
                    ExprType::Number(_rn) => match token.token_type {
                      TokenType::EqualEqual => Ok(ExprType::Boolean(false)),
                      TokenType::BangEqual => Ok(ExprType::Boolean(true)),
                      _ => Err(ExprEvalError::BinaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                    },
                    _ => Err(ExprEvalError::BinaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                  },
                  ExprType::Boolean(lb) => match right_value {
                    ExprType::Boolean(rb) => match token.token_type {
                      TokenType::EqualEqual => Ok(ExprType::Boolean(lb == rb)),
                      TokenType::BangEqual => Ok(ExprType::Boolean(lb != rb)),
                      _ => Err(ExprEvalError::BinaryError(format!("Invalid comparison for booleans.\n[line {}]", token.line)))
                    }
                    _ => Err(ExprEvalError::BinaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                  },
                  _ => Err(ExprEvalError::BinaryError(format!("Operands must be two numbers or two strings.\n[line {}]", token.line)))
                }
              },
              Err(e) => Err(e)
            }
          }, 
          Err(e) => Err(e)
        }
      },
      Expression::Logical(left, operator, right) => {
        match ExprEvaluator::evaluate(left, env) {
          Ok(l) => {
            if operator.token_type == TokenType::Or {
              if l.is_truthy() { return Ok(l); }
            } else {
              if !l.is_truthy() { return Ok(l); }
            }

            match ExprEvaluator::evaluate(right, env) {
              Ok(r) => Ok(r),
              Err(e) => Err(e)
            }
          },
          Err(e) => Err(e)
        }
      }
    }
  }
}