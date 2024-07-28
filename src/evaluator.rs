use crate::{expression::Expression, token::TokenType};

pub struct Evaluator;

pub enum RuntimeType {
  Boolean(bool),
  String(String),
  Number(f64),
  Nil()
}

impl RuntimeType {
  pub fn to_string(&self) -> String {
    match self {
      RuntimeType::Boolean(v) => v.to_string(),
      RuntimeType::String(v) => v.to_string(),
      RuntimeType::Number(v) => v.to_string(),
      RuntimeType::Nil() => String::from("nil")
    }
  }
}

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
      _ => RuntimeType::Nil()
    }
  }
}