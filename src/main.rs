use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

mod token; use token::Token;
mod parser; use parser::Parser;
mod expression;

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
            let tokens = &tokenize(filename, false);
            let expression = Parser::parse(tokens);

            match expression {
                Ok(e) => println!("{}", e.to_string()),
                Err(e) => {
                    eprintln!("{}", e.to_string());
                    process::exit(65);
                }
            };
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
