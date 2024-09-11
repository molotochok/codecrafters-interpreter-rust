use std::borrow::Cow;

#[derive(PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub name: &'static str,
    pub lexeme: Cow<'static, str>,
    pub literal: Cow<'static, str>,
    pub line: usize
}

pub enum TokenizeError {
    UnexpectedCharacter(String),
    UndeterminedString(String)
}

#[derive(PartialEq, Debug)]
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

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Token {
    // *** Single Character ***
    pub fn t_left_paren(line: usize) -> Token {
        Token { token_type: TokenType::LeftParen, name: "LEFT_PAREN", lexeme: Cow::Borrowed("("), literal: Cow::Borrowed("null"), line }
    }
    
    pub fn t_right_paren(line: usize) -> Token {
        Token { token_type: TokenType::RightParen, name: "RIGHT_PAREN", lexeme: Cow::Borrowed(")"), literal: Cow::Borrowed("null"), line  }
    }

    pub fn t_left_brace(line: usize) -> Token {
        Token { token_type: TokenType::LeftBrace, name: "LEFT_BRACE", lexeme: Cow::Borrowed("{"), literal: Cow::Borrowed("null"), line}
    }

    pub fn t_right_brace(line: usize) -> Token {
        Token { token_type: TokenType::RightBrace, name: "RIGHT_BRACE", lexeme: Cow::Borrowed("}"), literal: Cow::Borrowed("null"), line}
    }

    pub fn t_comma(line: usize) -> Token {
        Token { token_type: TokenType::Comma, name: "COMMA", lexeme: Cow::Borrowed(","), literal: Cow::Borrowed("null"), line}
    }

    pub fn t_dot(line: usize) -> Token {
        Token { token_type: TokenType::Dot, name: "DOT", lexeme: Cow::Borrowed("."), literal: Cow::Borrowed("null"), line}
    }

    pub fn t_plus(line: usize) -> Token {
        Token { token_type: TokenType::Plus, name: "PLUS", lexeme: Cow::Borrowed("+"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_star(line: usize) -> Token {
        Token { token_type: TokenType::Star, name: "STAR", lexeme: Cow::Borrowed("*"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_minus(line: usize) -> Token {
        Token { token_type: TokenType::Minus, name: "MINUS", lexeme: Cow::Borrowed("-"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_semicolon(line: usize) -> Token {
        Token { token_type: TokenType::Semicolon, name: "SEMICOLON", lexeme: Cow::Borrowed(";"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_space(line: usize) -> Token {
        Token { token_type: TokenType::Space, name: "SPACE", lexeme: Cow::Borrowed(""), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_tab(line: usize) -> Token {
        Token { token_type: TokenType::Tab, name: "TAB", lexeme: Cow::Borrowed(""), literal: Cow::Borrowed("null"), line }
    }

    // *** One or Two Characters ***
    pub fn t_less(line: usize) -> Token {
        Token { token_type: TokenType::Less, name: "LESS", lexeme: Cow::Borrowed("<"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_less_equal(line: usize) -> Token {
        Token { token_type: TokenType::LessEqual, name: "LESS_EQUAL", lexeme: Cow::Borrowed("<="), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_greater(line: usize) -> Token {
        Token { token_type: TokenType::Greater, name: "GREATER", lexeme: Cow::Borrowed(">"), literal: Cow::Borrowed("null"), line}
    }

    pub fn t_greater_equal(line: usize) -> Token {
        Token { token_type: TokenType::GreaterEqual, name: "GREATER_EQUAL", lexeme: Cow::Borrowed(">="), literal: Cow::Borrowed("null"), line}
    }

    pub fn t_bang(line: usize) -> Token {
        Token { token_type: TokenType::Bang, name: "BANG", lexeme: Cow::Borrowed("!"), literal: Cow::Borrowed("null"), line}
    }

    pub fn t_bang_equal(line: usize) -> Token {
        Token { token_type: TokenType::BangEqual, name: "BANG_EQUAL", lexeme: Cow::Borrowed("!="), literal: Cow::Borrowed("null"), line}
    }

    pub fn t_equal(line: usize) -> Token {
        Token { token_type: TokenType::Equal, name: "EQUAL", lexeme: Cow::Borrowed("="), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_equal_equal(line: usize) -> Token {
        Token { token_type: TokenType::EqualEqual, name: "EQUAL_EQUAL", lexeme: Cow::Borrowed("=="), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_slash(line: usize) -> Token {
        Token { token_type: TokenType::Slash, name: "SLASH", lexeme: Cow::Borrowed("/"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_comment(line: usize) -> Token {
        Token { token_type: TokenType::Comment, name: "COMMENT", lexeme: Cow::Borrowed("//"), literal: Cow::Borrowed("null"), line }
    }

    // *** Reserved words ***
    pub fn t_and(line: usize) -> Token {
        Token { token_type: TokenType::And, name: "AND", lexeme: Cow::Borrowed("and"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_class(line: usize) -> Token {
        Token { token_type: TokenType::Class, name: "CLASS", lexeme: Cow::Borrowed("class"), literal: Cow::Borrowed("null"), line }
    }
    
    pub fn t_else(line: usize) -> Token {
        Token { token_type: TokenType::Else, name: "ELSE", lexeme: Cow::Borrowed("else"), literal: Cow::Borrowed("null"), line }
    }
    
    pub fn t_false(line: usize) -> Token {
        Token { token_type: TokenType::False, name: "FALSE", lexeme: Cow::Borrowed("false"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_for(line: usize) -> Token {
        Token { token_type: TokenType::For, name: "FOR", lexeme: Cow::Borrowed("for"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_fun(line: usize) -> Token {
        Token { token_type: TokenType::Fun, name: "FUN", lexeme: Cow::Borrowed("fun"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_if(line: usize) -> Token {
        Token { token_type: TokenType::If, name: "IF", lexeme: Cow::Borrowed("if"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_nil(line: usize) -> Token {
        Token { token_type: TokenType::Nil, name: "NIL", lexeme: Cow::Borrowed("nil"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_or(line: usize) -> Token {
        Token { token_type: TokenType::Or, name: "OR", lexeme: Cow::Borrowed("or"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_return(line: usize) -> Token {
        Token { token_type: TokenType::Return, name: "RETURN", lexeme: Cow::Borrowed("return"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_super(line: usize) -> Token {
        Token { token_type: TokenType::Super, name: "SUPER", lexeme: Cow::Borrowed("super"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_this(line: usize) -> Token {
        Token { token_type: TokenType::This, name: "THIS", lexeme: Cow::Borrowed("this"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_true(line: usize) -> Token {
        Token { token_type: TokenType::True, name: "TRUE", lexeme: Cow::Borrowed("true"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_var(line: usize) -> Token {
        Token { token_type: TokenType::Var, name: "VAR", lexeme: Cow::Borrowed("var"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_while(line: usize) -> Token {
        Token { token_type: TokenType::While, name: "WHILE", lexeme: Cow::Borrowed("while"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_print(line: usize) -> Token {
        Token { token_type: TokenType::Print, name: "PRINT", lexeme: Cow::Borrowed("print"), literal: Cow::Borrowed("null"), line }
    }

    // *** Complex ***
    pub fn t_literal(lexeme: String, literal: String, line: usize) -> Token {
        Token { token_type: TokenType::String, name: "STRING", lexeme: Cow::Owned(lexeme), literal: Cow::Owned(literal), line }
    }

    pub fn t_number(lexeme: String, literal: String, line: usize) -> Token {
        Token { token_type: TokenType::Number, name: "NUMBER", lexeme: Cow::Owned(lexeme), literal: Cow::Owned(literal), line }
    }

    pub fn t_identifier(lexeme: String, line: usize) -> Token {
        Token { token_type: TokenType::Identifier, name: "IDENTIFIER", lexeme: Cow::Owned(lexeme), literal: Cow::Borrowed("null"), line }
    }
    
    // *** End ***
    pub fn t_eol(line: usize) -> Token {
        Token { token_type: TokenType::EOL, name: "EOL", lexeme: Cow::Borrowed("\n"), literal: Cow::Borrowed("null"), line }
    }

    pub fn t_eof(line: usize) -> Token {
        Token { token_type: TokenType::EOF, name: "EOF", lexeme: Cow::Borrowed(""), literal: Cow::Borrowed("null"), line}
    }


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
            match Token::from_bytes(bytes, i, line) {
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
    
        tokens.push(Token::t_eof(line));
    
        (tokens, errors)
    }

    fn from_bytes(bytes: &[u8], index: usize, line: usize) -> Result<Token, TokenizeError> {
        let char: char = bytes[index] as char;

        match char {
            '(' => Ok(Token::t_left_paren(line)),
            ')' => Ok(Token::t_right_paren(line)),
            '{' => Ok(Token::t_left_brace(line)),
            '}' => Ok(Token::t_right_brace(line)),
            ',' => Ok(Token::t_comma(line)),
            '.' => Ok(Token::t_dot(line)),
            '+' => Ok(Token::t_plus(line)),
            '*' => Ok(Token::t_star(line)),
            '-' => Ok(Token::t_minus(line)),
            ';' => Ok(Token::t_semicolon(line)),
            ' ' => Ok(Token::t_space(line)),
            '\t' => Ok(Token::t_tab(line)),
            '/' => Token::with_pair(bytes, index, Token::t_slash(line), Token::t_slash(line), Token::t_comment(line)),
            '<' => Token::with_pair(bytes, index, Token::t_less(line), Token::t_equal(line), Token::t_less_equal(line)),
            '>' => Token::with_pair(bytes, index, Token::t_greater(line), Token::t_equal(line), Token::t_greater_equal(line)),
            '!' => Token::with_pair(bytes, index, Token::t_bang(line), Token::t_equal(line), Token::t_bang_equal(line)),
            '=' => Token::with_pair(bytes, index, Token::t_equal(line), Token::t_equal(line), Token::t_equal_equal(line)),
            '"' => Token::with_literal(bytes, index, line),
            '0'..='9' => Token::with_number(bytes, index, line), 
            c if c.is_alphabetic() || c == '_' => Token::with_identifier(bytes, index, line),
            '\n' => Ok(Token::t_eol(line)),
            _ => Err(TokenizeError::UnexpectedCharacter(format!("Unexpected character: {}", char)))
        }
    }

    fn with_identifier(bytes: &[u8], index: usize, line: usize) -> Result<Token, TokenizeError> {
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
                if s == "and" { Ok(Token::t_and(line)) }
                else if s == "class" { Ok(Token::t_class(line)) }
                else if s == "else" { Ok(Token::t_else(line)) }
                else if s == "false" { Ok(Token::t_false(line)) }
                else if s == "for" { Ok(Token::t_for(line)) }
                else if s == "fun" { Ok(Token::t_fun(line)) }
                else if s == "if" { Ok(Token::t_if(line)) }
                else if s == "nil" { Ok(Token::t_nil(line)) }
                else if s == "or" { Ok(Token::t_or(line)) }
                else if s == "return" { Ok(Token::t_return(line)) }
                else if s == "super" { Ok(Token::t_super(line)) }
                else if s == "this" { Ok(Token::t_this(line)) }
                else if s == "true" { Ok(Token::t_true(line)) }
                else if s == "var" { Ok(Token::t_var(line)) }
                else if s == "while" { Ok(Token::t_while(line)) }
                else if s == "print" { Ok(Token::t_print(line)) }
                else { Ok(Token::t_identifier(s.to_owned(), line)) },
            Err(_) => Ok(Token::t_eol(line))
        }
    }

    fn with_number(bytes: &[u8], index: usize, line: usize) -> Result<Token, TokenizeError> {
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
                Ok(Token::t_number(s.to_owned(), literal, line))
            },
            Err(_) => Ok(Token::t_eol(line))
        }
    }

    fn with_literal(bytes: &[u8], index: usize, line: usize) -> Result<Token, TokenizeError> {
        let mut i = index + 1;
        while i < bytes.len() {
            let c = bytes[i] as char;

            if c == '"' {
                match std::str::from_utf8(&bytes[index..i + 1]) {
                    Ok(s) => return Ok(Token::t_literal(s.to_owned(), s[1..s.len() - 1].to_owned(), line)),
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