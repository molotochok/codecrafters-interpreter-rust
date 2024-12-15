use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::runtime::runtime_error::RuntimeError;
use crate::runtime::runtime_type::RuntimeType;
use crate::{expression::Expression, token::TokenType};

pub struct ExprEvaluator;

impl ExprEvaluator {
  pub fn evaluate(expression: &Expression, env: &Rc<RefCell<Environment>>) -> Result<RuntimeType, RuntimeError> {
    match expression {
      Expression::Nil() => Ok(RuntimeType::Nil()),
      Expression::Assign(token, expression) => {
        match ExprEvaluator::evaluate(expression, env) {
          Ok(value) => {
            match env.borrow_mut().assign(token.lexeme.to_string(), Rc::new(value.clone())) {
              Ok(()) => Ok(value),
              Err(e) => Err(e)
            }
          },
          Err(e) => Err(e)
        }
      },
      Expression::Identifier(token) => {
        match env.borrow().get(&token.lexeme.to_string()) {
          Some(v) => Ok((*v).clone()),
          None => Err(RuntimeError::UndefinedIdentifier(token.lexeme.to_string()))
        }
      },
      Expression::Literal(token) => {
        match token.token_type {
          TokenType::Nil => Ok(RuntimeType::Nil()),
          TokenType::True | TokenType::False => Ok(RuntimeType::Boolean(token.lexeme.parse::<bool>().unwrap())),
          TokenType::Number => Ok(RuntimeType::Number(token.literal.parse::<f64>().unwrap())),
          TokenType::String => Ok(RuntimeType::String(token.literal.to_string())),
          _ => Ok(RuntimeType::Nil())
        }
      },
      Expression::Grouping(e) => ExprEvaluator::evaluate(e, env),
      Expression::Unary(token, e) => {
        let value = ExprEvaluator::evaluate(e, env);

        match value {
          Ok(v) => {
            match token.token_type {
              TokenType::Bang => Ok(RuntimeType::Boolean(!v.is_truthy())),
              TokenType::Minus => match v {
                RuntimeType::Number(n) => Ok(RuntimeType::Number(-n)),
                _ => Err(RuntimeError::UnaryError(format!("Operand must be a number.\n[line {}]", token.line)))
              },
              _ => Ok(RuntimeType::Nil())
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
                  RuntimeType::Number(ln) => match right_value {
                    RuntimeType::Number(rn) => match token.token_type {
                      TokenType::Plus => Ok(RuntimeType::Number(ln + rn)),
                      TokenType::Minus => Ok(RuntimeType::Number(ln - rn)),
                      TokenType::Star => Ok(RuntimeType::Number(ln * rn)),
                      TokenType::Slash => Ok(RuntimeType::Number(ln / rn)),
                      TokenType::Greater => Ok(RuntimeType::Boolean(ln > rn)),
                      TokenType::GreaterEqual => Ok(RuntimeType::Boolean(ln >= rn)),
                      TokenType::Less => Ok(RuntimeType::Boolean(ln < rn)),
                      TokenType::LessEqual => Ok(RuntimeType::Boolean(ln <= rn)),
                      TokenType::EqualEqual => Ok(RuntimeType::Boolean(ln == rn)),
                      TokenType::BangEqual => Ok(RuntimeType::Boolean(ln != rn)),
                      _ => Ok(RuntimeType::Nil())
                    },
                    RuntimeType::String(_rs) => match token.token_type {
                      TokenType::EqualEqual => Ok(RuntimeType::Boolean(false)),
                      TokenType::BangEqual => Ok(RuntimeType::Boolean(true)),
                      _ => Err(RuntimeError::BinaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                    },
                    _ => Err(RuntimeError::BinaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                  },
                  RuntimeType::String(ls) => match right_value {
                    RuntimeType::String(rs) => match token.token_type {
                      TokenType::Plus => Ok(RuntimeType::String(ls + &rs)),
                      TokenType::EqualEqual => Ok(RuntimeType::Boolean(ls == rs)),
                      TokenType::BangEqual => Ok(RuntimeType::Boolean(ls != rs)),
                      _ => Err(RuntimeError::BinaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                    },
                    RuntimeType::Number(_rn) => match token.token_type {
                      TokenType::EqualEqual => Ok(RuntimeType::Boolean(false)),
                      TokenType::BangEqual => Ok(RuntimeType::Boolean(true)),
                      _ => Err(RuntimeError::BinaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                    },
                    _ => Err(RuntimeError::BinaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                  },
                  RuntimeType::Boolean(lb) => match right_value {
                    RuntimeType::Boolean(rb) => match token.token_type {
                      TokenType::EqualEqual => Ok(RuntimeType::Boolean(lb == rb)),
                      TokenType::BangEqual => Ok(RuntimeType::Boolean(lb != rb)),
                      _ => Err(RuntimeError::BinaryError(format!("Invalid comparison for booleans.\n[line {}]", token.line)))
                    }
                    _ => Err(RuntimeError::BinaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                  },
                  _ => Err(RuntimeError::BinaryError(format!("Operands must be two numbers or two strings.\n[line {}]", token.line)))
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
      },
      Expression::Call(callee, arguments) => {
        match ExprEvaluator::evaluate(callee, env) {
          Ok(eval_callee) => {
            let mut eval_args: Vec<Rc<RuntimeType>> = Vec::new();
            for arg in arguments {
              match ExprEvaluator::evaluate(arg, env) {
                Ok(v) => eval_args.push(Rc::new(v)),
                Err(e) => return Err(e)
              }
            }

            match eval_callee {
              RuntimeType::Function(function) => function.call(eval_args),
              _ => Err(RuntimeError::UndefinedIdentifier(callee.to_string()))
            }
          },
          Err(e) => Err(e)
        }
      }
    }
  }
}