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
        Ok(d) => declarations.push(d),
        Err(e) => return Err(e)
      }
    }

    Ok(declarations)
  }

  fn declaration<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Statement<'a>, ParserError> {
    match tokens[*index].token_type {
      TokenType::Var => StmtParser::var_declaration(tokens, index),
      _ => StmtParser::statement(tokens, index)
    }
  }

  fn var_declaration<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Statement<'a>, ParserError> {
    *index += 1;

    match ParserUtils::match_advance(tokens, index, &[TokenType::Identifier]) {
      Some(identifier) => match ParserUtils::match_advance(tokens, index, &[TokenType::Equal]) {
        Some(_equal) => {
          let value = StmtParser::expression(tokens, index, true);
          Ok(Statement::Var(identifier, Box::new(value.unwrap())))
        },
        None => {
          if &tokens[*index].token_type != &TokenType::Semicolon {
            return Err(ParserError::MissingToken(TokenType::Semicolon));
          }

          *index += 1;

          Ok(Statement::Var(identifier, Box::new(Expression::Nil())))
        },
      },
      None => Err(ParserError::ExpectExpression())
    }
  }

  fn statement<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<Statement<'a>, ParserError> {
    match tokens[*index].token_type {
      TokenType::EOL | TokenType::EOF => {
        *index += 1;
        Ok(Statement::Empty())
      },
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
            Ok(statement) => statements.push(statement),
            Err(e) => { return Err(e); }
          };
        };

        Ok(Statement::Block(Box::new(statements)))
      },
      TokenType::Print => {
        *index += 1;
        
        match StmtParser::expression(tokens, index, true) {
          Ok(expression) => Ok(Statement::Print(Box::new(expression))),
          Err(e) => Err(e),
        }
      },
      TokenType::If => {
        *index += 1;
        
        if !ParserUtils::matches(&tokens[*index], &[TokenType::LeftParen]) {
          return Err(ParserError::MissingToken(TokenType::LeftParen));
        }

        match StmtParser::expression(tokens, index, false) {
          Ok(condition) => {
            match StmtParser::statement(tokens, index) {
              Ok(then_stmt) => {
                Ok(Statement::If(Box::new(condition), Box::new(then_stmt), Box::new(Statement::Empty())))
              },
              Err(e) => Err(e)
            }
          }, 
          Err(e) => Err(e)
        }
      },
      _ => {
        match StmtParser::expression(tokens, index, true) {
          Ok(expression) => Ok(Statement::Expression(Box::new(expression))),
          Err(e) => Err(e),
        }
      }
    }
  }

  fn expression<'a>(tokens: &'a Vec<Token>, index: &mut usize, check_ending: bool) -> Result<Expression<'a>, ParserError> {
    let expression = ExprParser::expression(tokens, index);
          
    if expression.is_err() {
      return Err(expression.err().unwrap());
    }

    if check_ending {
      if &tokens[*index].token_type != &TokenType::Semicolon {
        return Err(ParserError::MissingToken(TokenType::Semicolon));
      }
      *index += 1;
    } 

    return expression;
  }
}
