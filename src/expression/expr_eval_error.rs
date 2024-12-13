pub enum ExprEvalError {
  UnaryError(String),
  BinaryError(String),
  UndefinedIdentifier(String),
}
impl ExprEvalError {
  pub fn to_string(&self) -> String {
    match self {
      ExprEvalError::UnaryError(m) => m.to_owned(),
      ExprEvalError::BinaryError(m) => m.to_owned(),
      ExprEvalError::UndefinedIdentifier(name) => format!("Identifier '{}' is undefined", name),
    }
  }
}
