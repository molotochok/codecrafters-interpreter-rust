use std::env;
use std::fs;
use std::io::{self, Write};

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
            tokenize(file_contents);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenize(str: String) {
    for (_i, c) in str.chars().enumerate() {
        let token_type = match c {
            '(' => TokenTypes::LeftParen,
            ')' => TokenTypes::RightParen,
            _ => panic!("An invalid token type")
        };

        let token = Token {
            token_type,
            lexeme: c.to_string(),
            literal: String::from("null")
        };

        println!("{}", token.to_str());
    }

    let token: Token = Token {
        token_type: TokenTypes::EOF,
        lexeme: String::from(""),
        literal: String::from("null")
    };

    println!("{}", token.to_str());
}

struct Token {
    token_type: TokenTypes,
    lexeme: String,
    literal: String
}

impl Token {
    fn to_str(&self) -> String {
        format!("{} {} {}", self.token_type.to_str(), self.lexeme, self.literal)
    }
}

enum TokenTypes {
    LeftParen, RightParen, EOF
}

impl TokenTypes {
    fn to_str(&self) -> &'static str {
        match self {
            TokenTypes::LeftParen => "LEFT_PAREN",
            TokenTypes::RightParen => "RIGHT_PAREN",
            TokenTypes::EOF => "EOF"
        }
    }
}
