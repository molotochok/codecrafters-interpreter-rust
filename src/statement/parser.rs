use crate::expression::Expression;
use crate::expression::parser::ExprParser;
use crate::parser::parser_error::ParserError;
use crate::parser::parser_utils::ParserUtils;
use crate::token::{Token, TokenType};
use crate::statement::Statement;

pub struct StmtParser;

impl StmtParser {
  pub fn parse<'a>(tokens: &'a Vec<Token>) -> Result<Vec<Statement<'a>>, ParserError> {
    let mut declarations: Vec<Statement> = Vec::new();
    let index = &mut 0;

    while *index < tokens.len() {
      match StmtParser::declaration(tokens, index) {
        Ok(option) => match option {
          Some(d) => declarations.push(d),
          None => break
        },
        Err(e) => return Err(e)
      }
    }

    Ok(declarations)
  }

  fn declaration<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Option<Statement<'a>>, ParserError> {
    match tokens[*index].token_type {
      TokenType::Var => StmtParser::var_declaration(tokens, index),
      _ => StmtParser::statement(tokens, index)
    }
  }

  fn var_declaration<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Option<Statement<'a>>, ParserError> {
    *index += 1;

    match ParserUtils::match_advance(tokens, index, &[TokenType::Identifier]) {
      Some(identifier) => match ParserUtils::match_advance(tokens, index, &[TokenType::Equal]) {
        Some(_equal) => {
          let value = StmtParser::expression(tokens, index);
          Ok(Some(Statement::Var(identifier, Box::new(value.unwrap()))))
        },
        None => {
          if &tokens[*index].token_type != &TokenType::Semicolon {
            return Err(ParserError::MissingToken(TokenType::Semicolon));
          }

          *index += 1;

          Ok(Some(Statement::Var(identifier, Box::new(Expression::Nil()))))
        },
      },
      None => Err(ParserError::ExpectExpression())
    }
  }

  fn statement<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Option<Statement<'a>>, ParserError> {
    match tokens[*index].token_type {
      TokenType::EOL | TokenType::EOF => Ok(None),
      TokenType::LeftBrace => {
        *index += 1;

        let mut statements: Vec<Statement<'a>> = Vec::new();

        loop {
          let token = &tokens[*index];

          if ParserUtils::matches(token, &[TokenType::RightBrace]) {
            *index += 1;
            break;
          }
          
          if ParserUtils::matches(token, &[TokenType::EOF]) {
            return Err(ParserError::MissingToken(TokenType::RightBrace));
          }

          match StmtParser::declaration(tokens, index) {
            Ok(declaration) => match declaration {
              Some(statement) => statements.push(statement),
              None => {}
            },
            Err(e) => { return Err(e); }
          };
        };

        Ok(Some(Statement::Block(Box::new(statements))))
      },
      TokenType::Print => {
        *index += 1;
        
        match StmtParser::expression(tokens, index) {
          Ok(expression) => Ok(Some(Statement::Print(Box::new(expression)))),
          Err(e) => Err(e),
        }
      },
      _ => {
        match StmtParser::expression(tokens, index) {
          Ok(expression) => Ok(Some(Statement::Expression(Box::new(expression)))),
          Err(e) => Err(e),
        }
      }
    }
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
