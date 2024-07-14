#[derive(PartialEq)]
pub struct Token {
    pub name: &'static str,
    pub lexeme: &'static str
}

impl Token {
    // *** Single Character ***
    pub const LEFT_PAREN: Token = Token {name: "LEFT_PAREN", lexeme: "("};
    pub const RIGHT_PAREN: Token = Token { name: "RIGHT_PAREN", lexeme: ")" };
    pub const LEFT_BRACE: Token = Token { name: "LEFT_BRACE", lexeme: "{" };
    pub const RIGHT_BRACE: Token = Token { name: "RIGHT_BRACE", lexeme: "}"};
    pub const COMMA: Token = Token { name: "COMMA", lexeme: "," };
    pub const DOT: Token = Token { name: "DOT", lexeme: "." };
    pub const PLUS: Token = Token { name: "PLUS", lexeme: "+" };
    pub const STAR: Token = Token { name: "STAR", lexeme: "*" };
    pub const MINUS: Token = Token { name: "MINUS", lexeme: "-" };
    pub const SEMICOLON: Token = Token { name: "SEMICOLON", lexeme: ";" };

    // *** One or Two Characters ***
    pub const LESS: Token = Token { name: "LESS", lexeme: "<" };
    pub const LESS_EQUAL: Token = Token { name: "LESS_EQUAL", lexeme: "<=" };
    pub const GREATER: Token = Token { name: "GREATER", lexeme: ">" };
    pub const GREATER_EQUAL: Token = Token { name: "GREATER_EQUAL", lexeme: ">=" };
    pub const BANG: Token = Token { name: "BANG", lexeme: "!" };
    pub const BANG_EQUAL: Token = Token { name: "BANG_EQUAL", lexeme: "!=" };
    pub const EQUAL: Token = Token { name: "EQUAL", lexeme: "=" };
    pub const EQUAL_EQUAL: Token = Token { name: "EQUAL_EQUAL", lexeme: "==" };
    pub const SLASH: Token = Token { name: "SLASH", lexeme: "/" };
    pub const COMMENT: Token = Token { name: "COMMENT", lexeme: "//" };
    
    // /n
    pub const EOL: Token = Token { name: "EOL", lexeme: "\n" };
    pub const EOF: Token = Token { name: "EOF", lexeme: "" };

    pub fn to_str(&self) -> String {
        format!("{} {} {}", self.name, self.lexeme, "null")
    }

    pub fn from_bytes(bytes: &[u8], index: usize) -> Result<Token, String> {
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
            '/' => Token::with_double(bytes, index, Token::SLASH, Token::SLASH, Token::COMMENT),
            '<' => Token::with_double(bytes, index, Token::LESS, Token::EQUAL, Token::LESS_EQUAL),
            '>' => Token::with_double(bytes, index, Token::GREATER, Token::EQUAL, Token::GREATER_EQUAL),
            '!' => Token::with_double(bytes, index, Token::BANG, Token::EQUAL, Token::BANG_EQUAL),
            '=' => Token::with_double(bytes, index, Token::EQUAL, Token::EQUAL, Token::EQUAL_EQUAL),
            '\n' => Ok(Token::EOL),
            _ => Err(String::from("An invalid token type"))
        }
    }

    fn with_double(bytes: &[u8], index: usize, first: Token, second: Token, double: Token) -> Result<Token, String> {
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