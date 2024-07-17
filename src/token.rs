use std::borrow::Cow;

#[derive(PartialEq)]
pub struct Token<'a> {
    pub name: &'static str,
    pub lexeme: &'a str,
    pub literal: Cow<'static, str>,
}

impl<'a> Token<'a> {
    // *** Single Character ***
    pub const LEFT_PAREN: Token<'a> = Token {name: "LEFT_PAREN", lexeme: "(", literal: Cow::Borrowed("null") };
    pub const RIGHT_PAREN: Token<'a> = Token { name: "RIGHT_PAREN", lexeme: ")", literal: Cow::Borrowed("null")};
    pub const LEFT_BRACE: Token<'a> = Token { name: "LEFT_BRACE", lexeme: "{", literal: Cow::Borrowed("null")};
    pub const RIGHT_BRACE: Token<'a> = Token { name: "RIGHT_BRACE", lexeme: "}", literal: Cow::Borrowed("null")};
    pub const COMMA: Token<'a> = Token { name: "COMMA", lexeme: ",", literal: Cow::Borrowed("null")};
    pub const DOT: Token<'a> = Token { name: "DOT", lexeme: ".", literal: Cow::Borrowed("null")};
    pub const PLUS: Token<'a> = Token { name: "PLUS", lexeme: "+", literal: Cow::Borrowed("null")};
    pub const STAR: Token<'a> = Token { name: "STAR", lexeme: "*", literal: Cow::Borrowed("null")};
    pub const MINUS: Token<'a> = Token { name: "MINUS", lexeme: "-", literal: Cow::Borrowed("null")};
    pub const SEMICOLON: Token<'a> = Token { name: "SEMICOLON", lexeme: ";", literal: Cow::Borrowed("null")};
    pub const SPACE: Token<'a> = Token { name: "SPACE", lexeme: "", literal: Cow::Borrowed("null")};
    pub const TAB: Token<'a> = Token { name: "TAB", lexeme: "", literal: Cow::Borrowed("null")};

    // *** One or Two Characters ***
    pub const LESS: Token<'a> = Token { name: "LESS", lexeme: "<", literal: Cow::Borrowed("null")};
    pub const LESS_EQUAL: Token<'a> = Token { name: "LESS_EQUAL", lexeme: "<=", literal: Cow::Borrowed("null")};
    pub const GREATER: Token<'a> = Token { name: "GREATER", lexeme: ">", literal: Cow::Borrowed("null")};
    pub const GREATER_EQUAL: Token<'a> = Token { name: "GREATER_EQUAL", lexeme: ">=", literal: Cow::Borrowed("null")};
    pub const BANG: Token<'a> = Token { name: "BANG", lexeme: "!", literal: Cow::Borrowed("null")};
    pub const BANG_EQUAL: Token<'a> = Token { name: "BANG_EQUAL", lexeme: "!=", literal: Cow::Borrowed("null")};
    pub const EQUAL: Token<'a> = Token { name: "EQUAL", lexeme: "=", literal: Cow::Borrowed("null")};
    pub const EQUAL_EQUAL: Token<'a> = Token { name: "EQUAL_EQUAL", lexeme: "==", literal: Cow::Borrowed("null")};
    pub const SLASH: Token<'a> = Token { name: "SLASH", lexeme: "/", literal: Cow::Borrowed("null")};
    pub const COMMENT: Token<'a> = Token { name: "COMMENT", lexeme: "//", literal: Cow::Borrowed("null")};

    // *** Reserved words ***
    pub const AND: Token<'a> = Token { name: "AND", lexeme: "and", literal: Cow::Borrowed("null") };
    pub const CLASS: Token<'a> = Token { name: "CLASS", lexeme: "class", literal: Cow::Borrowed("null") };
    pub const ELSE: Token<'a> = Token { name: "ELSE", lexeme: "else", literal: Cow::Borrowed("null") };
    pub const FALSE: Token<'a> = Token { name: "FALSE", lexeme: "false", literal: Cow::Borrowed("null") };
    pub const FOR: Token<'a> = Token { name: "FOR", lexeme: "for", literal: Cow::Borrowed("null") };
    pub const FUN: Token<'a> = Token { name: "FUN", lexeme: "fun", literal: Cow::Borrowed("null") };
    pub const IF: Token<'a> = Token { name: "IF", lexeme: "if", literal: Cow::Borrowed("null") };
    pub const NIL: Token<'a> = Token { name: "NIL", lexeme: "nil", literal: Cow::Borrowed("null") };
    pub const OR: Token<'a> = Token { name: "OR", lexeme: "or", literal: Cow::Borrowed("null") };
    pub const RETURN: Token<'a> = Token { name: "RETURN", lexeme: "return", literal: Cow::Borrowed("null") };
    pub const SUPER: Token<'a> = Token { name: "SUPER", lexeme: "super", literal: Cow::Borrowed("null") };
    pub const THIS: Token<'a> = Token { name: "THIS", lexeme: "this", literal: Cow::Borrowed("null") };
    pub const TRUE: Token<'a> = Token { name: "TRUE", lexeme: "true", literal: Cow::Borrowed("null") };
    pub const VAR: Token<'a> = Token { name: "VAR", lexeme: "var", literal: Cow::Borrowed("null") };
    pub const WHILE: Token<'a> = Token { name: "WHILE", lexeme: "while", literal: Cow::Borrowed("null") };

    // *** Complex ***
    pub fn new_literal(lexeme: &'a str, literal: String) -> Token<'a> {
        Token { name: "STRING", lexeme, literal: Cow::Owned(literal) }
    }

    pub fn new_number(lexeme: &'a str, literal: String) -> Token<'a> {
        Token { name: "NUMBER", lexeme, literal: Cow::Owned(literal) }
    }

    pub fn new_identifier(lexeme: &'a str) -> Token<'a> {
        Token { name: "IDENTIFIER", lexeme, literal: Cow::Borrowed("null") }
    }
    
    // /n
    pub const EOL: Token<'a> = Token { name: "EOL", lexeme: "\n", literal: Cow::Borrowed("null")};
    pub const EOF: Token<'a> = Token { name: "EOF", lexeme: "", literal: Cow::Borrowed("null")};

    pub fn tokenize(str: String) -> bool {
        let mut line = 1;
        let mut error_occurred = false;
    
        let bytes = str.as_bytes();
    
        let mut i = 0;
        while i < bytes.len() {
            match Token::from_bytes(bytes, i) {
                Ok(token) => {
                    if token.name == Token::EOL.name {
                        i += 1;
                        line += 1;
                        continue;
                    } else if token.name == Token::COMMENT.name {
                        while i < bytes.len() && bytes[i] as char != '\n' {
                            i += 1;
                        }
                        continue;
                    } else if token.name == Token::SPACE.name || token.name == Token::TAB.name {
                        i += 1;
                        continue;
                    } else {
                        i += if token.lexeme.len() > 0 { token.lexeme.len() } else { 1 };
                        println!("{}", token.to_str());
                    }
                },
                Err(error) => {
                    error_occurred = true;

                    match error {
                        Error::UnexpectedCharacter(msg) => {
                            eprintln!("[line {}] Error: {}", line, msg);
                            i += 1;
                        },
                        Error::UndeterminedString(msg) => {
                            eprintln!("[line {}] Error: {}", line, msg);
                            while i < bytes.len() && bytes[i] as char != '\n' {
                                i += 1;
                            }
                            line += 1;
                        }
                    }
                }
            };
        }
    
        println!("{}", Token::EOF.to_str());
    
        error_occurred
    }

    fn from_bytes(bytes: &[u8], index: usize) -> Result<Token, Error> {
        let char: char = bytes[index] as char;

        match char {
            '(' => Ok(Token::LEFT_PAREN),
            ')' => Ok(Token::RIGHT_PAREN),
            '{' => Ok(Token::LEFT_BRACE),
            '}' => Ok(Token::RIGHT_BRACE),
            ',' => Ok(Token::COMMA),
            '.' => Ok(Token::DOT),
            '+' => Ok(Token::PLUS),
            '*' => Ok(Token::STAR),
            '-' => Ok(Token::MINUS),
            ';' => Ok(Token::SEMICOLON),
            ' ' => Ok(Token::SPACE),
            '\t' => Ok(Token::TAB),
            '/' => Token::with_pair(bytes, index, Token::SLASH, Token::SLASH, Token::COMMENT),
            '<' => Token::with_pair(bytes, index, Token::LESS, Token::EQUAL, Token::LESS_EQUAL),
            '>' => Token::with_pair(bytes, index, Token::GREATER, Token::EQUAL, Token::GREATER_EQUAL),
            '!' => Token::with_pair(bytes, index, Token::BANG, Token::EQUAL, Token::BANG_EQUAL),
            '=' => Token::with_pair(bytes, index, Token::EQUAL, Token::EQUAL, Token::EQUAL_EQUAL),
            '"' => Token::with_literal(bytes, index),
            '0'..='9' => Token::with_number(bytes, index), 
            c if c.is_alphabetic() || c == '_' => Token::with_identifier(bytes, index),
            '\n' => Ok(Token::EOL),
            _ => Err(Error::UnexpectedCharacter(format!("Unexpected character: {}", char)))
        }
    }

    fn with_identifier(bytes: &[u8], index: usize) -> Result<Token, Error> {
        let mut i = index + 1;
        while i < bytes.len() {
            let c = bytes[i] as char;
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            i += 1;
        }

        match std::str::from_utf8(&bytes[index..i]) {
            Ok(s) => 
                if s == Token::AND.lexeme { Ok(Token::AND) }
                else if s == Token::CLASS.lexeme { Ok(Token::CLASS) }
                else if s == Token::ELSE.lexeme { Ok(Token::ELSE) }
                else if s == Token::FALSE.lexeme { Ok(Token::FALSE) }
                else if s == Token::FOR.lexeme { Ok(Token::FOR) }
                else if s == Token::FUN.lexeme { Ok(Token::FUN) }
                else if s == Token::IF.lexeme { Ok(Token::IF) }
                else if s == Token::NIL.lexeme { Ok(Token::NIL) }
                else if s == Token::OR.lexeme { Ok(Token::OR) }
                else if s == Token::RETURN.lexeme { Ok(Token::RETURN) }
                else if s == Token::SUPER.lexeme { Ok(Token::SUPER) }
                else if s == Token::THIS.lexeme { Ok(Token::THIS) }
                else if s == Token::TRUE.lexeme { Ok(Token::TRUE) }
                else if s == Token::VAR.lexeme { Ok(Token::VAR) }
                else if s == Token::WHILE.lexeme { Ok(Token::WHILE) }
                else { Ok(Token::new_identifier(s)) },
            Err(_) => Ok(Token::EOL)
        }
    }

    fn with_number(bytes: &[u8], index: usize) -> Result<Token, Error> {
        let mut i = index + 1;
        let mut dots = 0;
        while i < bytes.len() {
            let c = bytes[i] as char;

            if c == '.' {
                dots += 1;
                if dots > 1 {
                    break;
                }
            } else if bytes[i] < 48 || bytes[i] > 57 {
                break;
            }

            i += 1;
        }

        i -= 1;

        if bytes[i] as char == '.' {
            i -= 1;
        }

        match std::str::from_utf8(&bytes[index..i + 1]) {
            Ok(s) => {
                let number: f64 = s.parse().unwrap();
                let literal = if number.fract() == 0. { format!("{}.0", number) } else { number.to_string() };
                Ok(Token::new_number(s, literal))
            },
            Err(_) => Ok(Token::EOL)
        }
    }

    fn with_literal(bytes: &[u8], index: usize) -> Result<Token, Error> {
        let mut i = index + 1;
        while i < bytes.len() {
            let c = bytes[i] as char;

            if c == '\n' {
                break;
            }

            if c == '"' {
                match std::str::from_utf8(&bytes[index..i + 1]) {
                    Ok(s) => return Ok(Token::new_literal(s, s[1..s.len() - 1].to_owned())),
                    Err(_) => break
                };
            }

            i += 1;
        }

        Err(Error::UndeterminedString(String::from("Unterminated string.")))
    }

    fn with_pair(bytes: &[u8], index: usize, first: Token<'a>, second: Token<'a>, double: Token<'a>) -> Result<Token<'a>, Error> {
        if index >= bytes.len() - 1  {
            Ok(first)
        } else {
            match second.lexeme.chars().next() {
                Some(c) => {
                    if bytes[index + 1] as char == c {
                        Ok(double)
                    } else {
                        Ok(first)
                    }
                }
                None => Ok(first)
            }
        }
    }

    fn to_str(&self) -> String {
        format!("{} {} {}", self.name, self.lexeme, self.literal)
    }
}

enum Error {
    UnexpectedCharacter(String),
    UndeterminedString(String)
}