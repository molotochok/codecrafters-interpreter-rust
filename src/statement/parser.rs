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
          let value = StmtParser::expression(tokens, index, &Some(TokenType::Semicolon), false);
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
        
        match StmtParser::expression(tokens, index, &Some(TokenType::Semicolon), false) {
          Ok(expression) => Ok(Statement::Print(Box::new(expression))),
          Err(e) => Err(e),
        }
      },
      TokenType::If => {
        *index += 1;
        
        if !ParserUtils::matches(&tokens[*index], &[TokenType::LeftParen]) {
          return Err(ParserError::MissingToken(TokenType::LeftParen));
        }

        match StmtParser::expression(tokens, index, &None, false) {
          Ok(condition) => {
            match StmtParser::statement(tokens, index) {
              Ok(then_stmt) => {
                let mut else_stmt = Statement::Empty();
                
                match ParserUtils::match_advance(tokens, index, &[TokenType::Else]) {
                  Some(_else) => {
                    match StmtParser::statement(tokens, index) {
                      Ok(statement) => else_stmt = statement,
                      Err(e) => return Err(e)
                    }
                  },
                  None => {}
                }
                Ok(Statement::If(Box::new(condition), Box::new(then_stmt), Box::new(else_stmt)))
              },
              Err(e) => Err(e)
            }
          }, 
          Err(e) => Err(e)
        }
      },
      TokenType::While => {
        *index += 1;
        
        if !ParserUtils::matches(&tokens[*index], &[TokenType::LeftParen]) {
          return Err(ParserError::MissingToken(TokenType::LeftParen));
        }

        match StmtParser::expression(tokens, index, &None, false) {
          Ok(condition) => {
            match StmtParser::statement(tokens, index) {
              Ok(then_stmt) => Ok(Statement::While(Box::new(condition), Box::new(then_stmt))),
              Err(e) => Err(e)
            }
          }, 
          Err(e) => Err(e)
        }
      },
      TokenType::For => {
        *index += 1;

        match ParserUtils::match_advance(tokens, index, &[TokenType::LeftParen]) {
          Some(_l_paren_token) => {
            let decl_or_stmt; 
            
            if ParserUtils::matches(&tokens[*index], &[TokenType::Var]) {
              match StmtParser::var_declaration(tokens, index) {
                Ok(decl) => decl_or_stmt = decl,
                Err(e) => return Err(e)
              }
            } else {
              match StmtParser::expression(tokens, index, &Some(TokenType::Semicolon), true) {
                Ok(expr) => decl_or_stmt = Statement::Expression(Box::new(expr)),
                Err(e) => return Err(e)
              }
            }

            match StmtParser::expression(tokens, index, &Some(TokenType::Semicolon), true) {
              Ok(condition) => {
                match StmtParser::expression(tokens, index, &Some(TokenType::RightParen), true) {
                  Ok(increment) => {
                    match StmtParser::statement(tokens, index) {
                      Ok(statement) => {
                        let while_block = Statement::Block(Box::new(vec![statement, Statement::Expression(Box::new(increment))]));
                        let statements: Vec<Statement> = vec![decl_or_stmt, Statement::While(Box::new(condition), Box::new(while_block))];
                        Ok(Statement::Block(Box::new(statements)))
                      },
                      Err(e) => Err(e)
                    }
                  },
                  Err(e) => Err(e)
                }
              },
              Err(e) => Err(e)
            }
          },
          None => Err(ParserError::MissingToken(TokenType::LeftParen))
        }
      },
      _ => {
        match StmtParser::expression(tokens, index, &Some(TokenType::Semicolon), false) {
          Ok(expression) => Ok(Statement::Expression(Box::new(expression))),
          Err(e) => Err(e),
        }
      }
    }
  }

  fn expression<'a>(tokens: &'a Vec<Token>, index: &mut usize, ending_opt: &Option<TokenType>, optional: bool) -> Result<Expression<'a>, ParserError> {
    if optional {
      match ending_opt {
        Some(ending) => {
          if &tokens[*index].token_type == ending {
            *index += 1;
            return Ok(Expression::Nil());
          }
        },
        None => {}
      };
    }

    let expression = ExprParser::expression(tokens, index);
          
    if expression.is_err() {
      return Err(expression.err().unwrap());
    }

    match ending_opt {
      Some(ending) => {
        if &tokens[*index].token_type != ending {
          return Err(ParserError::MissingToken(TokenType::Semicolon));
        }
        *index += 1;
      },
      None => {}
    }

    return expression;
  }
}
