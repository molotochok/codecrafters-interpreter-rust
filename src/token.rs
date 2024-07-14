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
    pub const SLASH: Token = Token { name: "SLASH", lexeme: "/" };

    // *** One or Two Characters ***
    pub const LESS: Token = Token { name: "LESS", lexeme: "<" };
    pub const LESS_EQUAL: Token = Token { name: "LESS_EQUAL", lexeme: "<=" };
    pub const GREATER: Token = Token { name: "GREATER", lexeme: ">" };
    pub const GREATER_EQUAL: Token = Token { name: "GREATER_EQUAL", lexeme: ">=" };
    pub const BANG: Token = Token { name: "BANG", lexeme: "!" };
    pub const BANG_EQUAL: Token = Token { name: "BANG_EQUAL", lexeme: "!=" };
    pub const EQUAL: Token = Token { name: "EQUAL", lexeme: "=" };
    pub const EQUAL_EQUAL: Token = Token { name: "EQUAL_EQUAL", lexeme: "==" };
    
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
            '/' => Ok(Token::SLASH),
            '<' => Token::with_equal(bytes, index, Token::LESS, Token::LESS_EQUAL),
            '>' => Token::with_equal(bytes, index, Token::GREATER, Token::GREATER_EQUAL),
            '!' => Token::with_equal(bytes, index, Token::BANG, Token::BANG_EQUAL),
            '=' => Token::with_equal(bytes, index, Token::EQUAL, Token::EQUAL_EQUAL),
            '\n' => Ok(Token::EOL),
            _ => Err(String::from("An invalid token type"))
        }
    }

    fn with_equal(bytes: &[u8], index: usize, single: Token, double: Token) -> Result<Token, String> {
        if index >= bytes.len() - 1  {
            Ok(single)
        } else {
            match bytes[index + 1] as char {
                '=' => Ok(double),
                _  => Ok(single)
            }
        }
    }
}