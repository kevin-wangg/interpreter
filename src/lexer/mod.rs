mod tests;

use std::collections::HashMap;

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
        self.skip_whitespace();

        let token = match self.cur_char {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char(); // consume the second '='
                    Token::new(TokenType::Eq, "==")
                } else {
                    Token::new(TokenType::Assign, "=")
                }
            }
            '+' => Token::new(TokenType::Plus, "+"),
            '(' => Token::new(TokenType::LParen, "("),
            ')' => Token::new(TokenType::RParen, ")"),
            '{' => Token::new(TokenType::LBrace, "{"),
            '}' => Token::new(TokenType::RBrace, "}"),
            ',' => Token::new(TokenType::Comma, ","),
            ';' => Token::new(TokenType::Semicolon, ";"),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char(); // consume the second '='
                    Token::new(TokenType::NotEq, "!=")
                } else {
                    Token::new(TokenType::Bang, "!")
                }
            }
            '-' => Token::new(TokenType::Minus, "-"),
            '/' => Token::new(TokenType::Slash, "/"),
            '*' => Token::new(TokenType::Star, "*"),
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char(); // consume the second '='
                    Token::new(TokenType::LessEq, "<=")
                } else {
                    Token::new(TokenType::LArrow, "<")
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char(); // consume the second '='
                    Token::new(TokenType::GreaterEq, ">=")
                } else {
                    Token::new(TokenType::RArrow, ">")
                }
            }
            '\0' => Token::new(TokenType::Eof, ""),
            c => {
                let token = if c.is_alphabetic() || Self::is_underscore(c) {
                    let word = self.read_word();
                    let token_type = Self::lookup_ident(&word);
                    Token::new(token_type, &word)
                } else if c.is_numeric() {
                    let number = self.read_number();
                    Token::new(TokenType::Int, &number)
                } else {
                    Token::new(TokenType::Illegal, &c.to_string())
                };
                // Unread a character here because the functions used here (`read_word`, `read_number`)
                // reads until the first character NOT in the literal. Then the `read_char` call below
                // would then skip this character entirely, so we add a `unread_char` call here to
                // not skip it.
                self.unread_char();
                token
            }
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

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    /// Unreads a char by moving the position pointers back by 1.
    fn unread_char(&mut self) {
        if self.cur_position == 0 {
            self.cur_char = '\0';
            self.read_position = 0;
        } else {
            self.read_position = self.cur_position;
            self.cur_position -= 1;
            self.cur_char = self.input[self.cur_position];
        }
    }

    fn read_word(&mut self) -> String {
        let mut word = String::new();
        while self.cur_char.is_alphanumeric() || Self::is_underscore(self.cur_char) {
            word.push(self.cur_char);
            self.read_char();
        }
        word
    }

    // Note: Even though this function is called `read_number`, it returns a String.
    // This is because in the lexing stage, all token literals are String types.
    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while self.cur_char.is_numeric() {
            number.push(self.cur_char);
            self.read_char();
        }
        number
    }

    /// Returns TokenType::Ident if `word` is not a keyword in the Monkey
    /// programming language (eg. let, fun), or the corresponding token type otherwise.
    ///
    /// # Examples:
    ///
    /// Basic usage:
    ///
    /// ```
    /// assert!(lookup_ident("let") == TokenType::Let)
    /// assert!(lookup_ident("fun") == TokenType::Function)
    /// assert!(lookup_ident("skibidi") == TokenType::Ident)
    /// ```
    fn lookup_ident(word: &str) -> TokenType {
        let mut keywords = HashMap::new();
        keywords.insert("let", TokenType::Let);
        keywords.insert("fun", TokenType::Function);
        keywords.insert("true", TokenType::True);
        keywords.insert("false", TokenType::False);
        keywords.insert("if", TokenType::If);
        keywords.insert("else", TokenType::Else);
        keywords.insert("return", TokenType::Return);
        keywords.insert("null", TokenType::Null);
        keywords.insert("def", TokenType::Def);
        *keywords.get(word).unwrap_or(&TokenType::Ident)
    }

    fn is_underscore(c: char) -> bool {
        c == '_'
    }

    fn skip_whitespace(&mut self) {
        while self.cur_char.is_whitespace() {
            self.read_char();
        }
    }
}
