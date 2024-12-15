use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, statement::{evaluator::StmtEvaluator, Statement}, token::Token};

use super::{runtime_error::RuntimeError, runtime_type::RuntimeType};

pub struct RuntimeFunction {
  fun_name: String,
  args: Vec<Rc<Token>>,
  body: Box<Statement>,
  enclosing: Rc<RefCell<Environment>>
}

impl RuntimeFunction {
  pub fn new(fun_name: String, args: Vec<Rc<Token>>, body: Box<Statement>, enclosing: Rc<RefCell<Environment>>) -> Self {
    Self { fun_name, args, body, enclosing }
  }

  pub fn get_name(&self) -> String {
    self.fun_name.to_owned()
  }

  pub fn call(&self, args_values: Vec<Rc<RuntimeType>>) -> Result<RuntimeType, RuntimeError> { 
    if self.args.len() != args_values.len() {
      return Err(RuntimeError::WrongArgumentsForFunction(self.fun_name.clone(), self.args.len(), args_values.len()));
    }

    let local_env = Rc::new(RefCell::new(Environment::local(self.enclosing.clone())));

    for (arg_name, arg_value) in self.args.iter().zip(args_values.iter()) {
      local_env.borrow_mut().define(arg_name.lexeme.to_string(), arg_value.clone());
    }

    match StmtEvaluator::evaluate(&self.body, &local_env) {
      Ok(v) => Ok(v),
      Err(e) => Err(e)
    }
  }
}