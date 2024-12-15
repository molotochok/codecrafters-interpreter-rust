use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::runtime::{runtime_error::RuntimeError, runtime_type::RuntimeType};

pub struct Environment{
  enclosing: Option<Rc<RefCell<Environment>>>,
  map: Box<HashMap<String, Rc<RuntimeType>>>
}

impl Environment {
  pub fn global() -> Self {
    Self { enclosing: None, map: Box::new(HashMap::new())}
  }

  pub fn local(enclosing: Rc<RefCell<Environment>>) -> Self {
    Self { enclosing: Some(enclosing), map: Box::new(HashMap::new())}
  }

  // Example usage: var a = 2;
  pub fn define(&mut self, key: String, value: Rc<RuntimeType>) {
    self.map.insert(key, value);
  }

  // Example usage: a = 2;
  pub fn assign(&mut self, key: String, value: Rc<RuntimeType>) -> Result<(), RuntimeError> {
    if self.map.contains_key(&key) {
      self.define(key, value);
      return Ok(());
    }

    match &self.enclosing {
      Some(enclosing) => enclosing.borrow_mut().assign(key, value),
      None => Err(RuntimeError::UndefinedIdentifier(key))
    }
  }

  // Example usage: print a;
  pub fn get(&self, key: &String) -> Option<Rc<RuntimeType>> {
    match self.map.get(key) {
      Some(v) => Some(v.clone()),
      None => match &self.enclosing {
        Some(enclosing) => enclosing.borrow_mut().get(key),
        None => None
      }
    }
  }
}

