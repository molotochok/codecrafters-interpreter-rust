use std::borrow::Cow;

#[derive(PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub name: &'static str,
    pub lexeme: Cow<'static, str>,
    pub literal: Cow<'static, str>,
}

pub enum TokenizeError {
    UnexpectedCharacter(String),
    UndeterminedString(String)
}

#[derive(PartialEq)]
pub enum TokenType {
    // *** Single Character ***
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Plus, Star, Minus, Semicolon, Space, Tab,
    // *** One or Two Characters ***
    Less, LessEqual, Greater, GreaterEqual, Bang, BangEqual, Equal, EqualEqual, Slash, Comment,
    // *** Reserved words ***
    And, Class, Else, False, For, Fun, If, Nil, Or, Return, Super, This, True, Var, While, Print,
    // *** Complex ***
    String, Number, Identifier,
    // *** End ***
    EOL, EOF
}

impl Token {
    // *** Single Character ***
    pub const LEFT_PAREN: Token = Token { token_type: TokenType::LeftParen, name: "LEFT_PAREN", lexeme: Cow::Borrowed("("), literal: Cow::Borrowed("null") };
    pub const RIGHT_PAREN: Token = Token { token_type: TokenType::RightParen, name: "RIGHT_PAREN", lexeme: Cow::Borrowed(")"), literal: Cow::Borrowed("null")};
    pub const LEFT_BRACE: Token = Token { token_type: TokenType::LeftBrace, name: "LEFT_BRACE", lexeme: Cow::Borrowed("{"), literal: Cow::Borrowed("null")};
    pub const RIGHT_BRACE: Token = Token { token_type: TokenType::RightBrace, name: "RIGHT_BRACE", lexeme: Cow::Borrowed("}"), literal: Cow::Borrowed("null")};
    pub const COMMA: Token = Token { token_type: TokenType::Comma, name: "COMMA", lexeme: Cow::Borrowed(","), literal: Cow::Borrowed("null")};
    pub const DOT: Token = Token { token_type: TokenType::Dot, name: "DOT", lexeme: Cow::Borrowed("."), literal: Cow::Borrowed("null")};
    pub const PLUS: Token = Token { token_type: TokenType::Plus, name: "PLUS", lexeme: Cow::Borrowed("+"), literal: Cow::Borrowed("null")};
    pub const STAR: Token = Token { token_type: TokenType::Star, name: "STAR", lexeme: Cow::Borrowed("*"), literal: Cow::Borrowed("null")};
    pub const MINUS: Token = Token { token_type: TokenType::Minus, name: "MINUS", lexeme: Cow::Borrowed("-"), literal: Cow::Borrowed("null")};
    pub const SEMICOLON: Token = Token { token_type: TokenType::Semicolon, name: "SEMICOLON", lexeme: Cow::Borrowed(";"), literal: Cow::Borrowed("null")};
    pub const SPACE: Token = Token { token_type: TokenType::Space, name: "SPACE", lexeme: Cow::Borrowed(""), literal: Cow::Borrowed("null")};
    pub const TAB: Token = Token { token_type: TokenType::Tab, name: "TAB", lexeme: Cow::Borrowed(""), literal: Cow::Borrowed("null")};

    // *** One or Two Characters ***
    pub const LESS: Token = Token { token_type: TokenType::Less, name: "LESS", lexeme: Cow::Borrowed("<"), literal: Cow::Borrowed("null")};
    pub const LESS_EQUAL: Token = Token { token_type: TokenType::LessEqual, name: "LESS_EQUAL", lexeme: Cow::Borrowed("<="), literal: Cow::Borrowed("null")};
    pub const GREATER: Token = Token { token_type: TokenType::Greater, name: "GREATER", lexeme: Cow::Borrowed(">"), literal: Cow::Borrowed("null")};
    pub const GREATER_EQUAL: Token = Token { token_type: TokenType::GreaterEqual, name: "GREATER_EQUAL", lexeme: Cow::Borrowed(">="), literal: Cow::Borrowed("null")};
    pub const BANG: Token = Token { token_type: TokenType::Bang, name: "BANG", lexeme: Cow::Borrowed("!"), literal: Cow::Borrowed("null")};
    pub const BANG_EQUAL: Token = Token { token_type: TokenType::BangEqual, name: "BANG_EQUAL", lexeme: Cow::Borrowed("!="), literal: Cow::Borrowed("null")};
    pub const EQUAL: Token = Token { token_type: TokenType::Equal, name: "EQUAL", lexeme: Cow::Borrowed("="), literal: Cow::Borrowed("null")};
    pub const EQUAL_EQUAL: Token = Token { token_type: TokenType::EqualEqual, name: "EQUAL_EQUAL", lexeme: Cow::Borrowed("=="), literal: Cow::Borrowed("null")};
    pub const SLASH: Token = Token { token_type: TokenType::Slash, name: "SLASH", lexeme: Cow::Borrowed("/"), literal: Cow::Borrowed("null")};
    pub const COMMENT: Token = Token { token_type: TokenType::Comment, name: "COMMENT", lexeme: Cow::Borrowed("//"), literal: Cow::Borrowed("null")};

    // *** Reserved words ***
    pub const AND: Token = Token { token_type: TokenType::And, name: "AND", lexeme: Cow::Borrowed("and"), literal: Cow::Borrowed("null") };
    pub const CLASS: Token = Token { token_type: TokenType::Class, name: "CLASS", lexeme: Cow::Borrowed("class"), literal: Cow::Borrowed("null") };
    pub const ELSE: Token = Token { token_type: TokenType::Else, name: "ELSE", lexeme: Cow::Borrowed("else"), literal: Cow::Borrowed("null") };
    pub const FALSE: Token = Token { token_type: TokenType::False, name: "FALSE", lexeme: Cow::Borrowed("false"), literal: Cow::Borrowed("null") };
    pub const FOR: Token = Token { token_type: TokenType::For, name: "FOR", lexeme: Cow::Borrowed("for"), literal: Cow::Borrowed("null") };
    pub const FUN: Token = Token { token_type: TokenType::Fun, name: "FUN", lexeme: Cow::Borrowed("fun"), literal: Cow::Borrowed("null") };
    pub const IF: Token = Token { token_type: TokenType::If, name: "IF", lexeme: Cow::Borrowed("if"), literal: Cow::Borrowed("null") };
    pub const NIL: Token = Token { token_type: TokenType::Nil, name: "NIL", lexeme: Cow::Borrowed("nil"), literal: Cow::Borrowed("null") };
    pub const OR: Token = Token { token_type: TokenType::Or, name: "OR", lexeme: Cow::Borrowed("or"), literal: Cow::Borrowed("null") };
    pub const RETURN: Token = Token { token_type: TokenType::Return, name: "RETURN", lexeme: Cow::Borrowed("return"), literal: Cow::Borrowed("null") };
    pub const SUPER: Token = Token { token_type: TokenType::Super, name: "SUPER", lexeme: Cow::Borrowed("super"), literal: Cow::Borrowed("null") };
    pub const THIS: Token = Token { token_type: TokenType::This, name: "THIS", lexeme: Cow::Borrowed("this"), literal: Cow::Borrowed("null") };
    pub const TRUE: Token = Token { token_type: TokenType::True, name: "TRUE", lexeme: Cow::Borrowed("true"), literal: Cow::Borrowed("null") };
    pub const VAR: Token = Token { token_type: TokenType::Var, name: "VAR", lexeme: Cow::Borrowed("var"), literal: Cow::Borrowed("null") };
    pub const WHILE: Token = Token { token_type: TokenType::While, name: "WHILE", lexeme: Cow::Borrowed("while"), literal: Cow::Borrowed("null") };
    pub const PRINT: Token = Token { token_type: TokenType::Print, name: "PRINT", lexeme: Cow::Borrowed("print"), literal: Cow::Borrowed("null") };

    // *** Complex ***
    pub fn new_literal(lexeme: String, literal: String) -> Token {
        Token { token_type: TokenType::String, name: "STRING", lexeme: Cow::Owned(lexeme), literal: Cow::Owned(literal) }
    }

    pub fn new_number(lexeme: String, literal: String) -> Token {
        Token { token_type: TokenType::Number, name: "NUMBER", lexeme: Cow::Owned(lexeme), literal: Cow::Owned(literal) }
    }

    pub fn new_identifier(lexeme: String) -> Token {
        Token { token_type: TokenType::Identifier, name: "IDENTIFIER", lexeme: Cow::Owned(lexeme), literal: Cow::Borrowed("null") }
    }
    
    // *** End ***
    pub const EOL: Token = Token { token_type: TokenType::EOL, name: "EOL", lexeme: Cow::Borrowed("\n"), literal: Cow::Borrowed("null")};
    pub const EOF: Token = Token { token_type: TokenType::EOF, name: "EOF", lexeme: Cow::Borrowed(""), literal: Cow::Borrowed("null")};

    pub fn to_str(&self) -> String {
        format!("{} {} {}", self.name, self.lexeme, self.literal)
    }

    pub fn tokenize(str: &String) -> (Vec<Token>, Vec<String>) {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        let mut line = 1;
    
        let bytes = str.as_bytes();
    
        let mut i = 0;
        while i < bytes.len() {
            match Token::from_bytes(bytes, i) {
                Ok(token) => {
                    match token.token_type {
                        TokenType::EOL => {
                            i += 1;
                            line += 1;
                            continue;
                        },
                        TokenType::Comment => {
                            while i < bytes.len() && bytes[i] as char != '\n' {
                                i += 1;
                            }
                            continue;
                        },
                        TokenType::Space | TokenType::Tab => {
                            i += 1;
                            continue;
                        },
                        _ => {
                            i += if token.lexeme.len() > 0 { token.lexeme.len() } else { 1 };
                            tokens.push(token);
                        }
                    }
                },
                Err(error) => {
                    match error {
                        TokenizeError::UnexpectedCharacter(msg) => {
                            errors.push(format!("[line {}] Error: {}", line, msg));
                            
                            i += 1;
                        },
                        TokenizeError::UndeterminedString(msg) => {
                            errors.push(format!("[line {}] Error: {}", line, msg));

                            while i < bytes.len() && bytes[i] as char != '\n' {
                                i += 1;
                            }
                            line += 1;
                        }
                    }
                }
            };
        }
    
        tokens.push(Token::EOF);
    
        (tokens, errors)
    }

    fn from_bytes(bytes: &[u8], index: usize) -> Result<Token, TokenizeError> {
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
            _ => Err(TokenizeError::UnexpectedCharacter(format!("Unexpected character: {}", char)))
        }
    }

    fn with_identifier(bytes: &[u8], index: usize) -> Result<Token, TokenizeError> {
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
                else if s == Token::PRINT.lexeme { Ok(Token::PRINT) }
                else { Ok(Token::new_identifier(s.to_owned())) },
            Err(_) => Ok(Token::EOL)
        }
    }

    fn with_number(bytes: &[u8], index: usize) -> Result<Token, TokenizeError> {
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
                Ok(Token::new_number(s.to_owned(), literal))
            },
            Err(_) => Ok(Token::EOL)
        }
    }

    fn with_literal(bytes: &[u8], index: usize) -> Result<Token, TokenizeError> {
        let mut i = index + 1;
        while i < bytes.len() {
            let c = bytes[i] as char;

            if c == '\n' {
                break;
            }

            if c == '"' {
                match std::str::from_utf8(&bytes[index..i + 1]) {
                    Ok(s) => return Ok(Token::new_literal(s.to_owned(), s[1..s.len() - 1].to_owned())),
                    Err(_) => break
                };
            }

            i += 1;
        }

        Err(TokenizeError::UndeterminedString(String::from("Unterminated string.")))
    }

    fn with_pair(bytes: &[u8], index: usize, first: Token, second: Token, double: Token) -> Result<Token, TokenizeError> {
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
}