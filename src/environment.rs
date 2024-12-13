use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::expression::{expr_eval_error::ExprEvalError, expr_type::ExprType};

pub struct Environment{
  enclosing: Option<Rc<RefCell<Environment>>>,
  map: Box<HashMap<String, ExprType>>
}

impl Environment {
  pub fn global() -> Self {
    Self { enclosing: None, map: Box::new(HashMap::new())}
  }

  pub fn local(enclosing: Rc<RefCell<Environment>>) -> Self {
    Self { enclosing: Some(enclosing), map: Box::new(HashMap::new())}
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

    match &self.enclosing {
      Some(enclosing) => enclosing.borrow_mut().assign(key, value),
      None => Err(ExprEvalError::UndefinedIdentifier(key))
    }
  }

  // Example usage: print a;
  pub fn get(&self, key: &String) -> Option<RefCell<ExprType>> {
    match self.map.get(key) {
      Some(v) => Some(RefCell::new(v.clone())),
      None => match &self.enclosing {
        Some(enclosing) => enclosing.borrow_mut().get(key),
        None => None
      }
    }
  }
}

