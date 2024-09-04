pub mod token_type;

use token_type::TokenType;

pub struct Token {
    typ: TokenType,
    lexeme: String,
    literal: char,
    line: u32,
}

impl Token {
    pub fn new(typ: token_type::TokenType, lexeme: String, literal: char, line: u32) -> Self {
        Self {
            typ,
            lexeme,
            literal,
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.typ, self.lexeme, self.literal)
    }
}
