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
      }
      _ => RuntimeType::Nil()
    }
  }
}