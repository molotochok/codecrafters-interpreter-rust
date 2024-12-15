pub mod parser_error;
pub mod parser_utils;

use std::rc::Rc;

use parser_error::ParserError;

use crate::expression::parser::ExprParser;
use crate::statement::parser::StmtParser;
use crate::statement::Statement;
use crate::token::Token;
use crate::expression::Expression;

pub struct Parser;

impl Parser {
  pub fn parse_expression(tokens: &Vec<Rc<Token>>) -> Result<Expression, ParserError> {
    ExprParser::parse(tokens)
  }

  pub fn parse_statements(tokens: &Vec<Rc<Token>>) -> Result<Vec<Statement>, ParserError> {
    StmtParser::parse(tokens)
  }
}
