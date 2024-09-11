pub mod parser_error;
pub mod parser_utils;

use parser_error::ParserError;

use crate::expression::parser::ExprParser;
use crate::statement::parser::StmtParser;
use crate::statement::Statement;
use crate::token::Token;
use crate::expression::Expression;

pub struct Parser;

impl Parser {
  pub fn parse_expression<'a>(tokens: &'a Vec<Token>) -> Result<Expression<'a>, ParserError> {
    ExprParser::parse(tokens)
  }

  pub fn parse_statements<'a>(tokens: &'a Vec<Token>) -> Result<Vec<Statement<'a>>, ParserError> {
    StmtParser::parse(tokens)
  }
}
