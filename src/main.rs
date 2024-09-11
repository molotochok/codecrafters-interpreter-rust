use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use std::sync::Mutex;

mod token; use environment::Environment;
use expression::evaluator::ExprEvaluator;
use statement::evaluator::StmtEvaluator;
use token::Token;
mod parser; use parser::Parser;
mod statement; use statement::Statement;
mod expression; use expression::Expression;
mod environment;

static ENV: Mutex<Option<Environment>> = Mutex::new(None);

fn main() {
    *ENV.lock().unwrap() = Some(Environment::new());

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
            parse_expr(&tokens, true);
        },
        "evaluate" => {
            evaluate_expr(filename);
        },
        "run" => {
            run(filename)
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

fn parse_expr<'a>(tokens: &'a Vec<Token>, print_expr: bool) -> Expression<'a> {
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

fn evaluate_expr(filename: &String) {
    let tokens = tokenize(filename, false);
    let expression = parse_expr(&tokens, false);

    let result = ExprEvaluator::evaluate(&expression);

    match result {
        Ok(value) => println!("{}", value.to_string()),
        Err(e) => {
            eprintln!("{}", e.to_string());
            process::exit(70);
        }
    }
}

fn parse_stmt<'a>(tokens: &'a Vec<Token>, print: bool) -> Vec<Statement<'a>> {
    let result = Parser::parse_statements(tokens);

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

fn run(filename: &String) {
    let tokens = tokenize(filename, false);
    let statements = parse_stmt(&tokens, false);

    for statement in &statements {
        match StmtEvaluator::evaluate(statement) {
            Ok(r) => {
                match r {
                    Some(v) => println!("{}", v.to_string()),
                    None => {}
                }
            },
            Err(e) => {
                eprintln!("{}", e.to_string());
                process::exit(70);
            }
        }
    }
}
