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

  pub fn is_truthy(&self) -> bool {
    match self {
      RuntimeType::Nil() => false,
      RuntimeType::Boolean(b) => *b,
      _ => true
    }
  }
}