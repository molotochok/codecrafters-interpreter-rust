#[derive(Clone, Debug, PartialEq)]
pub enum ExprType {
  Boolean(bool),
  String(String),
  Number(f64),
  Nil()
}

impl ExprType {
  pub fn to_string(&self) -> String {
    match self {
      ExprType::Boolean(v) => v.to_string(),
      ExprType::String(v) => v.to_string(),
      ExprType::Number(v) => v.to_string(),
      ExprType::Nil() => String::from("nil")
    }
  }

  pub fn is_truthy(&self) -> bool {
    match self {
      ExprType::Nil() => false,
      ExprType::Boolean(b) => *b,
      _ => true
    }
  }
}