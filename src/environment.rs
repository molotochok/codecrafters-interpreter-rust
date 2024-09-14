use std::collections::HashMap;

use crate::expression::{expr_eval_error::ExprEvalError, expr_type::ExprType};

pub struct Environment {
  map: HashMap<String, ExprType>
}

impl Environment {
  pub fn new() -> Self {
    Self { map: HashMap::new() }
  }

  // Example usage: var a = 2;
  pub fn define(&mut self, key: String, value: ExprType) {
    self.map.insert(key, value);
  }

  // Example usage: a = 2;
  pub fn assign(&mut self, key: String, value: ExprType) -> Result<(), ExprEvalError> {
    if self.map.contains_key(&key) {
      self.define(key, value);
      return Ok(());
    }

    Err(ExprEvalError::UndefinedVariable(key))
  }

  // Example usage: print a;
  pub fn get(&mut self, key: &String) -> Option<&ExprType> {
    self.map.get(key)
  }
}

