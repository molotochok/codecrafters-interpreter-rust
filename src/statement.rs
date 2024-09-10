use crate::expression::Expression;

pub enum Statement<'a> {
  Print(Box<Expression<'a>>),
}

impl<'a> Statement<'a> {
  pub fn to_string(&self) -> String {
    match self {
      Statement::Print(expression) => {
        format!("Print: {}", expression.to_string())
      },
    }
  }
}