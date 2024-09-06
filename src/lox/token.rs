pub mod token_type;

use token_type::TokenType;

#[derive(Clone, Debug)]
pub struct Token<'src> {
    typ: TokenType,
    lexeme: &'src str,
    line: usize,
}

impl<'src> Token<'src> {
    pub fn new(typ: token_type::TokenType, lexeme: &'src str, line: usize) -> Self {
        Self { typ, lexeme, line }
    }
}

impl ToString for Token<'_> {
    fn to_string(&self) -> String {
        format!("{} {}", self.typ, self.lexeme)
    }
}
