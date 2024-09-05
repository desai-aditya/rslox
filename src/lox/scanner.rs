use crate::lox::token::Token;
use std::vec::Vec;

use super::token::token_type::TokenType;
use std::result::Result;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    line: usize,
    current: usize,
    start: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::<Token>::new(),
            line: 1,
            current: 0,
            start: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, (usize, String)> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), '\0', self.line));
        Ok(self.tokens.clone())
    }

    pub fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        c.expect("Character must be present at this index")
    }

    pub fn scan_token(&mut self) -> Result<(), (usize, String)> {
        let c = self.advance();
        match c {
            '(' => self.add_token_type(TokenType::LEFT_PAREN),
            ')' => self.add_token_type(TokenType::RIGHT_PAREN),
            '{' => self.add_token_type(TokenType::LEFT_BRACE),
            '}' => self.add_token_type(TokenType::RIGHT_BRACE),
            ',' => self.add_token_type(TokenType::COMMA),
            '.' => self.add_token_type(TokenType::DOT),
            '+' => self.add_token_type(TokenType::PLUS),
            '*' => self.add_token_type(TokenType::STAR),
            '-' => self.add_token_type(TokenType::MINUS),
            ';' => self.add_token_type(TokenType::SEMICOLON),
            '>' => match self.match_next_char('=') {
                true => self.add_token_type(TokenType::GREATER_EQUAL),
                false => self.add_token_type(TokenType::GREATER),
            },
            '<' => match self.match_next_char('=') {
                true => self.add_token_type(TokenType::LESS_EQUAL),
                false => self.add_token_type(TokenType::LESS),
            },
            '!' => match self.match_next_char('=') {
                true => self.add_token_type(TokenType::BANG_EQUAL),
                false => self.add_token_type(TokenType::BANG),
            },
            '=' => match self.match_next_char('=') {
                true => self.add_token_type(TokenType::EQUAL_EQUAL),
                false => self.add_token_type(TokenType::EQUAL),
            },
            '/' => match self.match_next_char('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token_type(TokenType::SLASH),
            },
            '\r' | ' ' | '\t' => (),
            '\n' => self.line += 1,
            c => return Err((self.line, format!("Character {c} is not recognized"))),
        }
        Ok(())
    }

    pub fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current)
            .expect("A character must be present at this index")
    }

    pub fn match_next_char(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c_src = self
            .source
            .chars()
            .nth(self.current)
            .expect("This character must be present at this index");
        if c != c_src {
            return false;
        }

        self.current += 1;
        true
    }

    pub fn add_token_type(&mut self, typ: TokenType) {
        self.add_token_literal(typ, '\0');
    }

    pub fn add_token_literal(&mut self, typ: TokenType, literal: char) {
        let substr = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(typ, substr, literal, self.line));
    }

    pub fn is_at_end(&self) -> bool {
        let current = self.current as usize;
        current >= self.source.len()
    }
}
