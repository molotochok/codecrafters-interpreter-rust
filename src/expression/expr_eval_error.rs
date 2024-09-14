pub enum ExprEvalError {
  UnaryError(String),
  BinaryError(String),
  UndefinedVariable(String),
}
impl ExprEvalError {
  pub fn to_string(&self) -> String {
    match self {
      ExprEvalError::UnaryError(m) => m.to_owned(),
      ExprEvalError::BinaryError(m) => m.to_owned(),
      ExprEvalError::UndefinedVariable(name) => format!("Variable '{}' is undefined", name),
    }
  }
}
