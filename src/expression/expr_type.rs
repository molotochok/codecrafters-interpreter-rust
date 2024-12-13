use std::rc::Rc;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub enum ExprType {
  Boolean(bool),
  String(String),
  Number(f64),
  Function(Rc<CallableFunction>),
  Nil()
}

impl ExprType {
  pub fn to_string(&self) -> String {
    match self {
      ExprType::Boolean(v) => v.to_string(),
      ExprType::String(v) => v.to_string(),
      ExprType::Number(v) => v.to_string(),
      ExprType::Nil() => String::from("nil"),
      ExprType::Function(_a) => String::from("<function>")
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

pub struct CallableFunction {
  func: Box<dyn Fn(Vec<ExprType>) -> ExprType>,
}

impl CallableFunction {
  pub fn new<F>(f: F) -> Self where F: Fn(Vec<ExprType>) -> ExprType + 'static {
    CallableFunction { func: Box::new(f) }
  }

  pub fn call(&self, args: Vec<ExprType>) -> ExprType { 
    (self.func)(args)
  }
}

impl Debug for CallableFunction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "<function>")
  }
}

impl PartialEq for CallableFunction {
  fn eq(&self, _other: &Self) -> bool { false }
}