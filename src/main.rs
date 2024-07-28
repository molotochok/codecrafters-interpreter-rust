use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

mod token; use token::Token;
mod parser; use parser::Parser;
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
            parse(&tokens, true);
        },
        "evaluate" => {
            let tokens = tokenize(filename, false);
            let expression = parse(&tokens, false);
            evaluate(&expression);
        },
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

fn parse<'a>(tokens: &'a Vec<Token>, print_expr: bool) -> Expression<'a> {
    let expression = Parser::parse(tokens);

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
