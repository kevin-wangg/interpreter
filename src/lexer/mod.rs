mod test;

use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    cur_position: usize,
    // Always points to 1 ahead of `cur_position`
    read_position: usize,
    cur_char: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            cur_position: 0,
            read_position: 0,
            cur_char: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.cur_char {
            '=' => Token::new(TokenType::Assign, "="),
            '+' => Token::new(TokenType::Plus, "+"),
            '(' => Token::new(TokenType::LParen, "("),
            ')' => Token::new(TokenType::RParen, ")"),
            '{' => Token::new(TokenType::LBrace, "{"),
            '}' => Token::new(TokenType::RBrace, "}"),
            ',' => Token::new(TokenType::Comma, ","),
            ';' => Token::new(TokenType::Semicolon, ";"),
            '\0' => Token::new(TokenType::Eof, ""),
            c => Token::new(TokenType::Illegal, &c.to_string())
        };
        self.read_char();
        token
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.cur_char = '\0';
        } else {
            self.cur_char = self.input[self.read_position];
            self.cur_position = self.read_position;
            self.read_position += 1;
        }
    }
}
