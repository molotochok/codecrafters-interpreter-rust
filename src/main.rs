use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

mod token; use token::Token;
mod parser; use parser::Parser;
mod statement; use statement::Statement;
mod expression; use expression::Expression;
mod evaluator; use evaluator::Evaluator;
mod runtime_type;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            tokenize(filename, true);
        },
        "parse" => {
            let tokens = tokenize(filename, false);
            parse_expression(&tokens, true);
        },
        "evaluate" => {
            let tokens = tokenize(filename, false);
            let expression = parse_expression(&tokens, false);
            evaluate(&expression);
        },
        "run" => {
            let tokens = tokenize(filename, false);
            let statements = parse_statement(&tokens, false);
            run(statements)
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenize(filename: &String, print_tokens: bool) -> Vec<Token> {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    let (tokens, errors) = Token::tokenize(&file_contents);

    for error in &errors {
        eprintln!("{}", error);
    }

    if print_tokens {
        for token in &tokens {
            println!("{}", token.to_str())
        }
    }

    if errors.len() > 0 {
        process::exit(65);
    }

    tokens
}

fn parse_expression<'a>(tokens: &'a Vec<Token>, print_expr: bool) -> Expression<'a> {
    let expression = Parser::parse_expression(tokens);

    match expression {
        Ok(e) => {
            if print_expr {
                println!("{}", e.to_string())
            }
            
            return e;
        },
        Err(e) => {
            eprintln!("{}", e.to_string());
            process::exit(65);
        }
    };
}

fn parse_statement<'a>(tokens: &'a Vec<Token>, print: bool) -> Vec<Statement<'a>> {
    let result = Parser::parse(tokens);

    match result {
        Ok(statements) => {
            if print {
                for statement in &statements {
                    println!("{}", statement.to_string())
                }
            }
            
            return statements;
        },
        Err(e) => {
            eprintln!("{}", e.to_string());
            process::exit(65);
        }
    };
}

fn run(statements: Vec<Statement>) {
    for statement in &statements {
        Evaluator::evaluate_s(statement)
    }
}

fn evaluate(expression: &Expression) {
    let result = Evaluator::evaluate(expression);

    match result {
        Ok(value) => println!("{}", value.to_string()),
        Err(e) => {
            eprintln!("{}", e.to_string());
            process::exit(70);
        }
    }
}
