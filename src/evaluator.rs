use crate::{expression::Expression, token::TokenType};
use crate::runtime_type::RuntimeType;

pub enum EvaluationError {
  UnaryError(String)
}
impl EvaluationError {
  pub fn to_string(&self) -> String {
    match self {
      EvaluationError::UnaryError(m) => m.to_owned()
    }
  }
}

pub struct Evaluator;
impl Evaluator {
  pub fn evaluate<'a>(expression: &'a Expression) -> Result<RuntimeType, EvaluationError> {
    match expression {
      Expression::Literal(token) => {
        match token.token_type {
          TokenType::Nil => Ok(RuntimeType::Nil()),
          TokenType::True | TokenType::False => Ok(RuntimeType::Boolean(token.lexeme.parse::<bool>().unwrap())),
          TokenType::Number => Ok(RuntimeType::Number(token.literal.parse::<f64>().unwrap())),
          TokenType::String => Ok(RuntimeType::String(token.literal.to_string())),
          _ => Ok(RuntimeType::Nil())
        }
      },
      Expression::Grouping(e) => Evaluator::evaluate(e),
      Expression::Unary(token, e) => {
        let value = Evaluator::evaluate(e);

        match value {
          Ok(v) => {
            match token.token_type {
              TokenType::Bang => Ok(RuntimeType::Boolean(!v.is_truthy())),
              TokenType::Minus => match v {
                RuntimeType::Number(n) => Ok(RuntimeType::Number(-n)),
                _ => Err(EvaluationError::UnaryError(format!("Operand must be a number.\n[line {}]", token.line)))
              },
              _ => Ok(RuntimeType::Nil())
            }
          },
          Err(e) => Err(e)
        }
      },
      Expression::Binary(left, token, right) => {
        let left_value_r = Evaluator::evaluate(left);
      
        match left_value_r {
          Ok(left_value) => {
            let right_value_r = Evaluator::evaluate(right);

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
                      _ => Err(EvaluationError::UnaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                    },
                    _ => Err(EvaluationError::UnaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                  },
                  RuntimeType::String(ls) => match right_value {
                    RuntimeType::String(rs) => match token.token_type {
                      TokenType::Plus => Ok(RuntimeType::String(ls + &rs)),
                      TokenType::EqualEqual => Ok(RuntimeType::Boolean(ls == rs)),
                      TokenType::BangEqual => Ok(RuntimeType::Boolean(ls != rs)),
                      _ => Err(EvaluationError::UnaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                    },
                    RuntimeType::Number(_rn) => match token.token_type {
                      TokenType::EqualEqual => Ok(RuntimeType::Boolean(false)),
                      TokenType::BangEqual => Ok(RuntimeType::Boolean(true)),
                      _ => Err(EvaluationError::UnaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                    },
                    _ => Err(EvaluationError::UnaryError(format!("Operands must be numbers.\n[line {}]", token.line)))
                  },
                  _ => Err(EvaluationError::UnaryError(format!("Operands must be two numbers or two strings.\n[line {}]", token.line)))
                }
              },
              Err(e) => Err(e)
            }
          }, 
          Err(e) => Err(e)
        }
      }
    }
  }
}