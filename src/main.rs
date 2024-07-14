use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

mod token;
use token::Token;
use token::TokenTypes;

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
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if tokenize(file_contents) {
                process::exit(65);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenize(str: String) -> bool {
    let mut line = 1;
    let mut error_occurred = false;

    for (_i, c) in str.chars().enumerate() {
        match TokenTypes::from_char(c) {
            Ok(token_type) => {
                if token_type == TokenTypes::EOL {
                    line += 1;
                    continue;
                }

                let token = Token {
                    token_type,
                    lexeme: c.to_string(),
                    literal: String::from("null")
                };
    
                println!("{}", token.to_str());
            },
            Err(_) => {
                error_occurred = true;
                eprintln!("[line {}] Error: Unexpected character: {}", line, c)
            }
        }
    }

    let token: Token = Token {
        token_type: TokenTypes::EOF,
        lexeme: String::from(""),
        literal: String::from("null")
    };

    println!("{}", token.to_str());

    error_occurred
}

