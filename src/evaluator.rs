use crate::{expression::Expression, token::TokenType};
use crate::runtime_type::RuntimeType;

pub struct Evaluator;

impl Evaluator {
  pub fn evaluate<'a>(expression: &'a Expression) -> RuntimeType {
    match expression {
      Expression::Literal(token) => {
        match token.token_type {
          TokenType::Nil => RuntimeType::Nil(),
          TokenType::True | TokenType::False => RuntimeType::Boolean(token.lexeme.parse::<bool>().unwrap()),
          TokenType::Number => RuntimeType::Number(token.literal.parse::<f64>().unwrap()),
          TokenType::String => RuntimeType::String(token.literal.to_string()),
          _ => RuntimeType::Nil()
        }
      },
      Expression::Grouping(e) => Evaluator::evaluate(e),
      Expression::Unary(token, e) => {
        let value = Evaluator::evaluate(e);

        match token.token_type {
          TokenType::Bang => RuntimeType::Boolean(!value.is_truthy()),
          TokenType::Minus => match value {
            RuntimeType::Number(n) => RuntimeType::Number(-n),
            _ => RuntimeType::Nil()
          },
          _ => RuntimeType::Nil()
        }
      },
      Expression::Binary(left, token, right) => {
        let left_value = Evaluator::evaluate(left);
        let right_value = Evaluator::evaluate(right);

        match left_value {
          RuntimeType::Number(ln) => match right_value {
            RuntimeType::Number(rn) => match token.token_type {
              TokenType::Plus => RuntimeType::Number(ln + rn),
              TokenType::Minus => RuntimeType::Number(ln - rn),
              TokenType::Star => RuntimeType::Number(ln * rn),
              TokenType::Slash => RuntimeType::Number(ln / rn),
              TokenType::Greater => RuntimeType::Boolean(ln > rn),
              TokenType::GreaterEqual => RuntimeType::Boolean(ln >= rn),
              TokenType::Less => RuntimeType::Boolean(ln < rn),
              TokenType::LessEqual => RuntimeType::Boolean(ln <= rn),
              TokenType::EqualEqual => RuntimeType::Boolean(ln == rn),
              TokenType::BangEqual => RuntimeType::Boolean(ln != rn),
              _ => RuntimeType::Nil()
            },
            RuntimeType::String(_rs) => match token.token_type {
              TokenType::EqualEqual => RuntimeType::Boolean(false),
              TokenType::BangEqual => RuntimeType::Boolean(true),
              _ => RuntimeType::Nil()
            },
            _ => RuntimeType::Nil()
          },
          RuntimeType::String(ls) => match right_value {
            RuntimeType::String(rs) => match token.token_type {
              TokenType::Plus =>  RuntimeType::String(ls + &rs),
              TokenType::EqualEqual => RuntimeType::Boolean(ls == rs),
              TokenType::BangEqual => RuntimeType::Boolean(ls != rs),
              _ => RuntimeType::Nil()
            },
            RuntimeType::Number(_rn) => match token.token_type {
              TokenType::EqualEqual => RuntimeType::Boolean(false),
              TokenType::BangEqual => RuntimeType::Boolean(true),
              _ => RuntimeType::Nil()
            },
            _ => RuntimeType::Nil()
          },
          _ => RuntimeType::Nil()
        }
      }
    }
  }
}