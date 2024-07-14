use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

mod token;
use token::Token;

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

    let bytes = str.as_bytes();

    let mut i = 0;
    let mut step;

    while i < bytes.len() {
        match Token::from_bytes(bytes, i) {
            Ok(token) => {
                match token {
                    Token::EOL => {
                        line += 1;
                        continue;
                    },
                    Token::COMMENT => {
                        while i < bytes.len() && bytes[i] as char != '\n' {
                            i += 1;
                            line += 1;
                        }
                        continue;
                    },
                    _ => {
                        step = token.lexeme.len();
                        println!("{}", token.to_str());
                    }
                }
            },
            Err(_) => {
                error_occurred = true;
                step = 1;
                eprintln!("[line {}] Error: Unexpected character: {}", line, bytes[i] as char)
            }
        };
        i += step;
    }

    println!("{}", Token::EOF.to_str());

    error_occurred
}

