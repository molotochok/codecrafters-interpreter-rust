use std::collections::HashMap;

use crate::expression::expr_type::ExprType;

pub struct Environment {
  map: HashMap<String, ExprType>
}

impl Environment {
  pub fn new() -> Self {
    Self { map: HashMap::new() }
  }

  pub fn set(&mut self, key: String, value: ExprType) {
    self.map.insert(key, value);
  }

  pub fn get(&mut self, key: &String) -> Option<&ExprType> {
    self.map.get(key)
  }
}

