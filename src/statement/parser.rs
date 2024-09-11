use crate::expression::Expression;
use crate::expression::parser::ExprParser;
use crate::parser::parser_error::ParserError;
use crate::token::{Token, TokenType};
use crate::statement::Statement;

pub struct StmtParser;

impl StmtParser {
  pub fn parse<'a>(tokens: &'a Vec<Token>) -> Result<Vec<Statement<'a>>, ParserError> {
    let mut statements: Vec<Statement> = Vec::new();
    let index = &mut 0;

    while *index < tokens.len() {
      match tokens[*index].token_type {
        TokenType::EOL | TokenType::EOF => break,
        TokenType::Print => {
          *index += 1;
          let expression = StmtParser::expression(tokens, index);
          statements.push(Statement::Print(Box::new(expression.unwrap())));
        }
        _ => {
          let expression = StmtParser::expression(tokens, index);
          statements.push(Statement::Expression(Box::new(expression.unwrap())));
        }
      }
    }

    Ok(statements)
  }

  fn expression<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Expression<'a>, ParserError> {
    let expression = ExprParser::expression(tokens, index);
          
    if expression.is_err() {
      return Err(expression.err().unwrap());
    }

    if &tokens[*index].token_type != &TokenType::Semicolon {
      return Err(ParserError::MissingToken(TokenType::Semicolon))
    }

    *index += 1;
    return expression;
  }
}
