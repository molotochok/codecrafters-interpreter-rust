use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, expression::evaluator::ExprEvaluator, runtime::{runtime_error::RuntimeError, runtime_function::RuntimeFunction, runtime_type::RuntimeType}};

use super::Statement;

pub struct StmtEvaluator;

impl StmtEvaluator {
  pub fn evaluate(statement: &Statement, env: &Rc<RefCell<Environment>>) -> Result<RuntimeType, RuntimeError> {
    match statement {
      Statement::Empty() => Ok(RuntimeType::Nil()),
      Statement::Print(e) => {
        match ExprEvaluator::evaluate(e, env) {
          Ok(t) => {
            println!("{}", t.to_string());
            Ok(RuntimeType::Nil())
          },
          Err(e) => Err(RuntimeError::StatementError(e.to_string()))
        }
      },
      Statement::Expression(e) => {
        match ExprEvaluator::evaluate(e, env) {
          Ok(_r) => Ok(RuntimeType::Nil()),
          Err(e) => Err(RuntimeError::StatementError(e.to_string()))
        }
      },
      Statement::Var(token, e) => {
        match ExprEvaluator::evaluate(e, env) {
          Ok(t) => {
            env.borrow_mut().define(token.lexeme.to_string(), Rc::new(t));
            return Ok(RuntimeType::Nil());
          },
          Err(e) => Err(RuntimeError::StatementError(e.to_string()))
        }
      },
      Statement::Function(func_name, args_names, body) => {
        let fun = RuntimeFunction::new(func_name.lexeme.to_string(), args_names.clone(), body.clone(), env.clone());
        let fun_type = RuntimeType::Function(Rc::new(fun));
        env.borrow_mut().define(func_name.lexeme.to_string(), Rc::new(fun_type));
        return Ok(RuntimeType::Nil());
      },
      Statement::Block(statements) => {
        let local_env = Rc::new(RefCell::new(Environment::local(env.clone())));
        
        for statement in statements.as_ref() {
          let res = StmtEvaluator::evaluate(&statement, &Rc::clone(&local_env));
          
          if res.is_err() {
            return res;
          }
        }

        return Ok(RuntimeType::Nil());
      },
      Statement::If(expr, then_stmt, else_stmt) => {
        match ExprEvaluator::evaluate(expr, &env) {
          Ok(condition) => {
            StmtEvaluator::evaluate(if condition.is_truthy() { &then_stmt } else { &else_stmt }, &env)
          },
          Err(e) => Err(RuntimeError::StatementError(e.to_string()))
        }
      },
      Statement::While(expr, stmt) => {
        loop {
          match ExprEvaluator::evaluate(&expr, &env) {
            Ok(condition) => {
              if condition.is_truthy() {
                let res = StmtEvaluator::evaluate(&stmt, &env);
  
                if res.is_err() {
                  return res;
                }
              } else {
                return Ok(RuntimeType::Nil());
              }
            },
            Err(e) => { return Err(RuntimeError::StatementError(e.to_string())); }
          } 
        }
      },
      Statement::Native(fun) => {
        return fun();
      }
    }
  }
}