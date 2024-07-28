use crate::expression::Expression;

pub struct Evaluator;

impl Evaluator {
  pub fn evaluate<'a>(expression: &'a Expression) -> String {
    match expression {
      Expression::Literal(literal) => literal.to_owned(),
      _ => String::new()
    }
  }
}