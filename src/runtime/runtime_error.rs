use super::runtime_type::RuntimeType;

pub enum RuntimeError {
  UnaryError(String),
  BinaryError(String),
  UndefinedIdentifier(String),
  WrongArgumentsForFunction(String, usize, usize),
  StatementError(String),
  ReturnValue(RuntimeType)
}
impl RuntimeError {
  pub fn to_string(&self) -> String {
    match self {
      RuntimeError::UnaryError(m) => m.to_owned(),
      RuntimeError::BinaryError(m) => m.to_owned(),
      RuntimeError::UndefinedIdentifier(name) => format!("Identifier '{}' is undefined", name),
      RuntimeError::WrongArgumentsForFunction(func_name, expected_len, got_len, ) => format!("Function '{}' was called with wrong number of arguments. Expected: {}, got: {}", func_name, expected_len, got_len),
      RuntimeError::StatementError(e) => format!("Statement failure: {}", e.to_string()),
      RuntimeError::ReturnValue(v) => format!("Return error with value: {}", v.to_string())
    }
  }
}
